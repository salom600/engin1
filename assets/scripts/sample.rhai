// Sample Rhai script — Rust-like syntax for game logic.
// Attach to an entity via Add Component → Rhai Script.
//
// Rhai is a small, fast scripting language with Rust-like syntax.
// It's interpreted at runtime, so you can edit and reload without recompiling.

// Called once when the game starts (play mode).
fn on_start(entity) {
    print("Hello from Rhai! Entity: " + entity);

    // Set the entity's initial velocity (if it has a RigidBody)
    // entity.set_velocity(0.0, 5.0, 0.0);
}

// Called every frame while the game is playing.
// `dt` is the delta time in seconds.
fn on_update(entity, dt) {
    // Rotate the entity slowly
    let speed = 1.0;
    let transform = entity.get_transform();
    transform.rotation_y = transform.rotation_y + speed * dt;
    entity.set_transform(transform);

    // Move forward when pressing W
    if input.is_key_down("W") {
        let pos = entity.get_position();
        pos.z = pos.z - speed * dt;
        entity.set_position(pos);
    }
}

// Called when the entity collides with another entity.
fn on_collision(entity, other) {
    print("Collision! " + entity + " hit " + other);
}

// Called once when the game stops.
fn on_stop(entity) {
    print("Goodbye from Rhai! Entity: " + entity);
}
