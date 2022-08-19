#[derive(Clone)]
pub enum ShipType {
    AircraftCarrier,
    Battleship,
    WeaponsPlatform,
    Submarine,
    PatrolBoat,
}

pub struct Ship {
    pub ship_type: ShipType,
    pub size: usize,
    pub hits: usize,
    pub name: String,
}

impl Ship {
    pub fn new(ship_type: ShipType) -> Self {
        let size = match ship_type {
            ShipType::AircraftCarrier => 5,
            ShipType::Battleship => 4,
            ShipType::WeaponsPlatform => 3,
            ShipType::Submarine => 3,
            ShipType::PatrolBoat => 2,
        };

        let name = match ship_type {
            ShipType::AircraftCarrier => "Aircraft Carrier",
            ShipType::Battleship => "Battleship",
            ShipType::WeaponsPlatform => "Weapons Platform",
            ShipType::Submarine => "Submarine",
            ShipType::PatrolBoat => "Patrol Boat",
        }.to_string();

        Self {
            ship_type,
            size,
            hits: 0,
            name,
        }
    }
}
