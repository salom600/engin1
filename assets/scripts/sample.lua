-- Sample Lua script for the Bevy editor.
-- Attach this script to an entity via the Inspector panel.
-- The script can read / write the entity's Transform, RigidBody, etc.

-- Called once when the script is attached.
function on_start(entity)
    print("Hello from Lua script on entity: " .. tostring(entity))
end

-- Called every frame while the game is playing.
function on_update(entity, dt)
    -- Rotate the entity slowly
    local transform = entity:get_transform()
    transform.rotation.y = transform.rotation.y + dt * 1.0
    entity:set_transform(transform)
end

-- Called when the script is detached.
function on_stop(entity)
    print("Script stopped on entity: " .. tostring(entity))
end
