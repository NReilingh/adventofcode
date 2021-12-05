// Day two

pub fn sub_position(input: Vec<String>) -> (u32, u32) {
    const DEBUG: bool = false;

    enum SubMovement {
        Forward(u32),
        Down(u32),
        Up(u32),
    }

    let movements: Vec<SubMovement> = input.iter()
        .map(|item| {
            let line: Vec<&str> = item.split_whitespace().collect();
            let mvt = line[0];
            let magnitude = line[1].parse::<u32>().unwrap();
            match mvt as &str {
                "forward" => SubMovement::Forward(magnitude),
                "up" => SubMovement::Up(magnitude),
                "down" => SubMovement::Down(magnitude),
                _ => unreachable!(),
            }
        }).collect();

    struct SubPosition {
        depth: u32,
        distance: u32,
        aim: u32,
    }

    impl SubPosition {
        fn product(&self) -> u32 {
            self.depth * self.distance
        }
    }

    let mut naive_position = SubPosition {
        depth: 0,
        distance: 0,
        aim: 0,
    };

    let mut position = SubPosition {
        depth: 0,
        distance: 0,
        aim: 0,
    };

    for movement in movements {
        match movement {
            SubMovement::Forward(mag) => {
                naive_position.distance += mag;
                position.distance += mag;
                position.depth += position.aim * mag;
            },
            SubMovement::Up(mag) => {
                naive_position.depth -= mag;
                position.aim -= mag;
            },
            SubMovement::Down(mag) => {
                naive_position.depth += mag;
                position.aim += mag;
            }
        }
    }

    let naive_result = naive_position.product();
    let position_result = position.product();

    if DEBUG {
        println!("The naive position vector product is {}", naive_result);
        println!("The position vector product is {}", position_result);
    }

    (naive_result, position_result)
}
