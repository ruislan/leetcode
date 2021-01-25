use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 直接模拟路线即可
        // 每次看下一步是否是障碍
        // 如果是障碍，那么就不用继续执行这个命令了
        // 如果不是障碍，就在这个方向上继续前进
        enum Direction { North, South, East, West }
        let mut obstacle_set = std::collections::HashSet::new();
        for obstacle in obstacles {
            obstacle_set.insert((obstacle[0], obstacle[1]));
        }
        let mut direction = Direction::North;
        let mut position = (0, 0);
        for cmd in commands {
            match cmd {
                -1 => direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                },
                -2 => direction = match direction {
                    Direction::North => Direction::West,
                    Direction::West => Direction::South,
                    Direction::South => Direction::East,
                    Direction::East => Direction::North,
                },
                _ => {
                    for i in 0..cmd {
                        let next_position = match direction {
                            Direction::North => (position.0, position.1 + 1),
                            Direction::East => (position.0 + 1, position.1),
                            Direction::South => (position.0, position.1 - 1),
                            Direction::West => (position.0 - 1, position.1),
                        };
                        if obstacle_set.contains(&next_position) {
                            break;
                        }
                        position = next_position;
                    }
                }
            }
        }
        position.0 * position.0 + position.1 * position.1
    }
}