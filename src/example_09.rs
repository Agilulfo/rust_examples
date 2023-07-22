pub fn run() {
    // Little game that consist in a storage of n items.
    // Players in turn roll a d6 dice and  take the resulting
    // number of items only if that number of items is available.
    // If not enough items are available in the storage, the player has to pay
    // back up to that number of items if he posses any.
    // The game ends when a player rolls a dice that empties the storage.
    // nobody wins... :-p

    let mut game = game::Game::new();
    game.run();
}

mod game {
    use super::player::Player;
    use super::reserve::Reserve;
    use std::collections::VecDeque;

    pub struct Game {
        player_queue: VecDeque<Player>,
        reserve: Reserve,
    }

    impl Game {
        pub fn new() -> Game {
            // Using here a queue to simulate a circular list.
            // players are removed from one side and inserted back
            // on the other side to keep the turn order
            // curious to know if there is an easy way
            // to have a more efficient circular list
            // maybe it is possible to build a custom iterator
            // that loops over and over a simple Vec.

            let mut queue = VecDeque::new();

            // TODO.. here I've initally tried to pass a string slice (&str)
            // but got a compiler error related to lifetime
            // maybe you can pass such value but a variable needs to be defined
            queue.push_back(Player::new(String::from("Alice")));
            queue.push_back(Player::new(String::from("Bob")));
            let reserve = Reserve::new(10);

            Game {
                player_queue: queue,
                reserve,
            }
        }
        pub fn run(&mut self) {
            while self.reserve.value() != 0 {
                let player = self.next_player();
                let reserve = self.mut_reserve();

                player.play(reserve);
            }
        }

        fn next_player(&mut self)-> &mut Player {
            let player = self.player_queue.pop_front().unwrap();
            self.player_queue.push_back(player);
            self.player_queue.back_mut().unwrap()
        }

        fn mut_reserve(&mut self) -> &mut Reserve {
            &mut self.reserve
        }
    }
}

mod reserve {
    pub struct Reserve {
        resources: u8,
    }

    impl Reserve {
        pub fn new(count: u8) -> Reserve {
            println!("creating a reserve with {} resources", count);
            Reserve { resources: count }
        }

        pub fn take(&mut self, count: u8) -> Option<u8> {
            if count > self.resources {
                None
            } else {
                self.resources = self.resources - count;
                println!("take {}", count);
                Some(count)
            }
        }

        pub fn give(&mut self, count: u8) {
            println!("give {}", count);
            self.resources = self.resources + count
        }

        pub fn value(&self) -> u8 {
            self.resources
        }
    }

    #[cfg(test)]
    mod reserve_tests {
        #[test]
        fn can_take() {
            let mut reserve = super::Reserve::new(10);
            assert!(reserve.take(5).unwrap() == 5);
            assert!(reserve.value() == 5);
        }

        #[test]
        fn can_add() {
            let mut reserve = super::Reserve::new(10);
            reserve.give(5);
            assert!(reserve.value() == 15);
        }
    }
}

mod player {
    use rand::Rng;
    use std::cmp;

    pub struct Player {
        name: String,
        score: u8,
    }

    impl Player {
        pub fn new(name: String) -> Player {
            Player { score: 0, name }
        }

        pub fn play(&mut self, reserve: &mut super::reserve::Reserve) {
            let mut value = roll_d6();

            println!("{} rolled {}", self.name, value);
            match reserve.take(value) {
                Some(value) => {
                    self.score += value;
                }
                None => {
                    value = cmp::min(self.score, value);
                    reserve.give(value);
                    self.score = self.score - value;
                }
            }
        }
    }
    fn roll_d6() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..5) + 1
    }

    #[cfg(test)]
    mod player_test {
        #[test]
        fn play() {
            let mut player = super::Player::new(String::from("test"));
            // interesting that super::super works!
            let mut reserve = super::super::reserve::Reserve::new(10);
            let original_value = reserve.value();
            player.play(&mut reserve);
            assert!(reserve.value() != original_value);
        }
    }
}
