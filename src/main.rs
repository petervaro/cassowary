/* INFO **
** INFO */

extern crate cassowary;
use cassowary::cl::point::Point;
use cassowary::cl::variable::{Variable, GenericVariable};

/*----------------------------------------------------------------------------*/
fn main()
{
    let mut var: GenericVariable<f32>;
    for _ in range(0u, 3)
    {
        var = Variable::new(None, None);
        println!("{}", var);
    }

    let mut pt: Point<uint>;
    for i in range(0u, 5)
    {
        pt = Point::new(i, i*i);
        println!("{}", pt);
    }

}
