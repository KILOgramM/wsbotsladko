use crate::multirolefix::load_btag_data_multirole;
use crate::HeroInfoReq;
use crate::OwData;

#[test]
fn show_role_prce_resulte(){
	let mut reqconf = HeroInfoReq::empty();
	reqconf.rating = true;
	let answer = load_btag_data_multirole(
		"Sladko#21716".to_string(),
		"EU".to_string(),
		"PC".to_string(),
		reqconf
	);
	if let OwData::Full(bdata) = answer{
		let rating = bdata.rating;
//		println!("{:#?}",rating);
		assert_eq!((None,None,Some(3008u16)),(rating.tank,rating.damage,rating.support))
	}
	else {
		panic!("show_role_prce_resulte profile is notfound or close");
	}
}
