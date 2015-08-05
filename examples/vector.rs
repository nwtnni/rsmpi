extern crate mpi;

use mpi::traits::*;
use mpi::datatype::{UserDatatype, View};
use mpi::topology::Rank;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    let next_rank = if rank + 1 < size { rank + 1 } else { 0 };
    let previous_rank = if rank - 1 >= 0 { rank - 1 } else { size - 1 };

    let mut b1 = (1..).map(|x| rank * x).take(6).collect::<Vec<_>>();
    let mut b2 = std::iter::repeat(-1).take(6).collect::<Vec<_>>();
    println!("Rank {} sending message: {:?}.", rank, b1);
    world.barrier();

    let t = UserDatatype::vector(2, 2, 3, Rank::equivalent_datatype());
    let status;
    {
        let mut v1 = unsafe { View::with_count_and_datatype(&mut b1[..], 1, &t) };
        let mut v2 = unsafe { View::with_count_and_datatype(&mut b2[..], 1, &t) };
        status = world.send_receive_into(&mut v1, next_rank, &mut v2, previous_rank);
    }

    println!("Rank {} received message: {:?}, status: {:?}.", rank, b2, status);
    world.barrier();

    let b3 = (1..).map(|x| if x % 3 == 0 { -1 } else { previous_rank * x })
        .take(6).collect::<Vec<_>>();
    assert_eq!(b3, b2);
}
