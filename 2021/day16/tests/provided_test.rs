use day16::calculator::*;
use day16::parser::*;

#[test]
fn provided_tests() {
    println!("C200B40A82 == 3");
    assert_eq!(calculate(&mut parse(&"C200B40A82".to_string())).unwrap(), 3);

    println!("04005AC33890 == 54");
    assert_eq!(
        calculate(&mut parse(&"04005AC33890".to_string())).unwrap(),
        54
    );

    println!("880086C3E88112 == 7");
    assert_eq!(
        calculate(&mut parse(&"880086C3E88112".to_string())).unwrap(),
        7
    );

    println!("CE00C43D881120 == 9");
    assert_eq!(
        calculate(&mut parse(&"CE00C43D881120".to_string())).unwrap(),
        9
    );

    println!("D8005AC2A8F0 == 1");
    assert_eq!(
        calculate(&mut parse(&"D8005AC2A8F0".to_string())).unwrap(),
        1
    );

    println!("F600BC2D8F == 0");
    assert_eq!(calculate(&mut parse(&"F600BC2D8F".to_string())).unwrap(), 0);

    println!("9C005AC2F8F0 == 0");
    assert_eq!(
        calculate(&mut parse(&"9C005AC2F8F0".to_string())).unwrap(),
        0
    );

    let tokens = parse(&"9C0141080250320F1802104A08".to_string());
    pretty_print(&tokens);

    println!("9C0141080250320F1802104A08 == 1");
    assert_eq!(
        calculate(&mut parse(&"9C0141080250320F1802104A08".to_string())).unwrap(),
        1
    );
}
