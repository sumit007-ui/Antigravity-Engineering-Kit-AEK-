# Project Rule: Robotics & Autonomous Systems (ROS 2)

## Standards & Toolchains
- **Framework**: ROS 2 Humble / Iron / Jazzy.
- **Node Architecture**: Use lifecycle nodes (`rclcpp_lifecycle` / `rclpy`) for deterministic startup, activation, and shutdown.
- **Transforms & Timing**: Use TF2 buffer with exact message header timestamps; avoid hardcoded frame identifiers.
- **Safety Invariants**: Enforce bounded emergency stop (E-stop) timeouts and rate-limited command velocity publish limits.
- **Simulation & Testing**: Validate URDF kinematics and launch files using Gazebo and `launch_testing`.
