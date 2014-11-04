/* INFO **
** INFO */

extern crate cassowary;
use cassowary::cl::variable::Variable;

/*----------------------------------------------------------------------------*/
fn main()
{
    for _ in range(0u, 3)
    {
        println!("{}", Variable::new("", ""));
    }
}
