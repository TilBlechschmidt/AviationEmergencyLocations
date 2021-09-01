use super::*;

#[derive(Debug)]
pub enum DubinPath {
    CSC(Arc, Tangent, Arc),
    CCC(Arc, Arc, Arc),
}

impl DubinPath {
    pub fn length(&self) -> Distance {
        match self {
            DubinPath::CSC(departure, travel, arrival) => {
                departure.length() + travel.length() + arrival.length()
            }
            DubinPath::CCC(arc1, arc2, arc3) => arc1.length() + arc2.length() + arc3.length(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            DubinPath::CSC(departure, _, arrival) => {
                format!(
                    "{}S{}",
                    departure.circle.direction, arrival.circle.direction
                )
            }
            DubinPath::CCC(arc1, arc2, arc3) => {
                format!(
                    "{}{}{}",
                    arc1.circle.direction, arc2.circle.direction, arc3.circle.direction
                )
            }
        }
    }
}
