
use tgdscad::*;

fn main() {
    let e = 10.;
    let s = 50.;
    let b = D3::beveled_box(XYZ(e,e,s), 1.)
        .translate(s/2.-e, s/2.-e, -s/2.)
        .iter_rotate(XYZ(0.,0.,90.), 4)
        .union()
        .iter_rotate(XYZ(0.,90.,0.), 2)
        .union()
        .iter_rotate(XYZ(90.,0.,0.), 2)
        .union()
        ;
    let sqrt2 = f64::powf(2.,0.5);
    let t_edge = (s-2.*e)*sqrt2;
    let t = D3::cuboid(t_edge, 0.01, 0.01)
        .translate(-t_edge/2., 0., t_edge/2./sqrt2)
        .add_map(|x| x.rotate(180., 0., 90.))
        .hull()
        ;

    println!("{}", &b.add(t.translate(s,0.,0.)));
    // println!("{}", &t);
}