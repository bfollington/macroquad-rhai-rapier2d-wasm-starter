use macroquad::prelude::*;
use rapier2d::prelude::*;

const PLAYER_SIZE: f32 = 30.0;
const JUMP_FORCE: f32 = 100000.0;
const MOVE_SPEED: f32 = 100.0;
const PLATFORM_WIDTH: f32 = 100.0;
const PLATFORM_HEIGHT: f32 = 20.0;
const PLATFORM_SPEED: f32 = 500.0;

struct Player {
    body: RigidBodyHandle,
    collider: ColliderHandle,
    is_on_platform: bool,
    platform_velocity: Vector<Real>,
}

struct MovingPlatform {
    body: RigidBodyHandle,
    collider: ColliderHandle,
    start_x: f32,
    end_x: f32,
    direction: f32,
}

struct PhysicsContext {
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

fn update_physics(rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet, physics_context: &mut PhysicsContext) {
    physics_context.physics_pipeline.step(
        &physics_context.gravity,
        &physics_context.integration_parameters,
        &mut physics_context.island_manager,
        &mut physics_context.broad_phase,
        &mut physics_context.narrow_phase,
        rigid_body_set,
        collider_set,
        &mut physics_context.impulse_joint_set,
        &mut physics_context.multibody_joint_set,
        &mut physics_context.ccd_solver,
        None,
        &(),
        &(),
    );
}


fn setup_physics() -> (RigidBodySet, ColliderSet, PhysicsContext) {
    let rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // Create ground
    let ground_collider = ColliderBuilder::cuboid(screen_width() / 2.0, 10.0)
        .translation(vector![screen_width() / 2.0, screen_height() - 10.0])
        .build();
    collider_set.insert(ground_collider);

    let physics_context = PhysicsContext {
        gravity: vector![0.0, 200.81],
        integration_parameters: IntegrationParameters::default(),
        physics_pipeline: PhysicsPipeline::new(),
        island_manager: IslandManager::new(),
        broad_phase: BroadPhase::new(),
        narrow_phase: NarrowPhase::new(),
        impulse_joint_set: ImpulseJointSet::new(),
        multibody_joint_set: MultibodyJointSet::new(),
        ccd_solver: CCDSolver::new(),
    };

    (rigid_body_set, collider_set, physics_context)
}

fn setup_player(rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) -> Player {
    let player_body = RigidBodyBuilder::dynamic()
        .translation(vector![screen_width() / 2.0, screen_height() / 2.0])
        .build();
    let player_body_handle = rigid_body_set.insert(player_body);
    let player_collider = ColliderBuilder::cuboid(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0)
        .restitution(0.0)
        .friction(0.5)
        .build();
    let player_collider_handle = collider_set.insert_with_parent(
        player_collider,
        player_body_handle,
        rigid_body_set,
    );
    Player {
        body: player_body_handle,
        collider: player_collider_handle,
        is_on_platform: false,
        platform_velocity: vector![0.0, 0.0],
    }
}

fn setup_platforms(rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) -> Vec<MovingPlatform> {
    let mut platforms = vec![];
    for i in 0..3 {
        let y = screen_height() - 100.0 - i as f32 * 150.0;
        let start_x = 100.0;
        let end_x = start_x + 300.0;
        let platform_body = RigidBodyBuilder::kinematic_velocity_based()
            .translation(vector![start_x, y])
            .build();
        let platform_body_handle = rigid_body_set.insert(platform_body);
        let platform_collider = ColliderBuilder::cuboid(PLATFORM_WIDTH / 2.0, PLATFORM_HEIGHT / 2.0)
            .friction(0.5)
            .build();
        let platform_collider_handle = collider_set.insert_with_parent(
            platform_collider,
            platform_body_handle,
            rigid_body_set,
        );
        platforms.push(MovingPlatform {
            body: platform_body_handle,
            collider: platform_collider_handle,
            start_x,
            end_x,
            direction: 1.0,
        });
    }
    platforms
}

fn text(s: rhai::ImmutableString, x: f32, y: f32, size: f32, color: Color) {
    draw_text(&s, x, y, size, color);
}


fn setup_rhai() -> rhai::Engine {
    let mut engine = rhai::Engine::new();
    engine.register_fn("text", text);
    engine.register_fn("fps", get_fps);
    engine.register_fn("screen_width",  screen_width);
    engine.register_fn("screen_height",  screen_height);
    engine
}


#[macroquad::main("Platformer")]
async fn main() -> Result<(), Box<rhai::EvalAltResult>>  {
    let (mut rigid_body_set, mut collider_set, mut physics_context) = setup_physics();
    let mut player = setup_player(&mut rigid_body_set, &mut collider_set);
    let mut platforms = setup_platforms(&mut rigid_body_set, &mut collider_set);
    let mut engine = setup_rhai();
    let mut scope = rhai::Scope::new();
    let state = rhai::Map::new();
    scope.push_constant("BLACK", BLACK)
    .push("state", state);

    let source = load_string("script/input.rhai").await.unwrap();

    loop {
        // Render
        clear_background(WHITE);

        // Handle input
        let mut x_movement = 0.0;
        if is_key_down(KeyCode::Left) {
            x_movement -= MOVE_SPEED;
        }
        if is_key_down(KeyCode::Right) {
            x_movement += MOVE_SPEED;
        }
        if is_key_pressed(KeyCode::Space) {
            if let Some(player_body) = rigid_body_set.get_mut(player.body) {
                player_body.apply_impulse(vector![0.0, -JUMP_FORCE], true);
            }
        }

        engine.run_with_scope(&mut scope,source.as_str())?;

        // Reset player's platform state
        player.is_on_platform = false;
        player.platform_velocity = vector![0.0, 0.0];

        // Check for collisions between player and platforms
        for platform in &platforms {
            if let (Some(player_body), Some(platform_body)) = (rigid_body_set.get(player.body), rigid_body_set.get(platform.body)) {
                let player_pos = player_body.translation();
                let platform_pos = platform_body.translation();
                
                if player_pos.y + PLAYER_SIZE / 2.0 >= platform_pos.y - PLATFORM_HEIGHT / 2.0 &&
                   player_pos.y - PLAYER_SIZE / 2.0 <= platform_pos.y + PLATFORM_HEIGHT / 2.0 &&
                   player_pos.x + PLAYER_SIZE / 2.0 >= platform_pos.x - PLATFORM_WIDTH / 2.0 &&
                   player_pos.x - PLAYER_SIZE / 2.0 <= platform_pos.x + PLATFORM_WIDTH / 2.0
                {
                    player.is_on_platform = true;
                    player.platform_velocity = platform_body.linvel().clone();
                    break;
                }
            }
        }

        // Apply movement to player
        if let Some(player_body) = rigid_body_set.get_mut(player.body) {
            let mut new_velocity = vector![x_movement, player_body.linvel().y];
            if player.is_on_platform {
                new_velocity += player.platform_velocity;
            }
            player_body.set_linvel(new_velocity, true);
        }

        // Move platforms
        for platform in &mut platforms {
            if let Some(platform_body) = rigid_body_set.get_mut(platform.body) {
                let position = platform_body.translation();
                if position.x < platform.start_x || position.x > platform.end_x {
                    platform.direction *= -1.0;
                }
                platform_body.set_linvel(vector![PLATFORM_SPEED * platform.direction, 0.0], true);
            }
        }

        update_physics(&mut rigid_body_set, &mut collider_set, &mut physics_context);

        // Draw ground
        draw_rectangle(
            0.0,
            screen_height() - 20.0,
            screen_width(),
            20.0,
            GRAY,
        );

        // Draw platforms
        for platform in &platforms {
            if let Some(platform_body) = rigid_body_set.get(platform.body) {
                let position = platform_body.translation();
                draw_rectangle(
                    position.x - PLATFORM_WIDTH / 2.0,
                    position.y - PLATFORM_HEIGHT / 2.0,
                    PLATFORM_WIDTH,
                    PLATFORM_HEIGHT,
                    GREEN,
                );
            }
        }

        // Draw player
        if let Some(player_body) = rigid_body_set.get(player.body) {
            let position = player_body.translation();
            draw_rectangle(
                position.x - PLAYER_SIZE / 2.0,
                position.y - PLAYER_SIZE / 2.0,
                PLAYER_SIZE,
                PLAYER_SIZE,
                BLUE,
            );
        }

        next_frame().await
    }
}