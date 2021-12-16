
pub fn car_to_binary(car: char) -> String {
    match car {
       '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!()
    }.to_string()
}

pub fn to_binary(line: String) -> String{
    line.chars().map(|car|
        car_to_binary(car)
    ).collect::<Vec<_>>().join("")
}

#[derive(Debug, Clone)]
enum PacketData {
    Literal(String),
    Operator(Vec<Packet>)
}

#[derive(Debug, Clone)]
struct Packet {
    id: String,
    version: String,
    data: PacketData,
}

fn pow2(pow: usize) -> u128 {
    let mut result : u128 = 1;
    for _ in 0..pow {
        result = result * 2;
    }
    result
}

impl Packet {
    fn version(&self) -> u128 {
        binary_to_dec(&self.version) + match &self.data {
            PacketData::Literal(_) => 0,
            PacketData::Operator(sub_packets) => sub_packets.iter().map(|packet| packet.version()).sum()
        }
    }

    fn value(&self) -> u128 {
        match &self.data {
            PacketData::Literal(literal) => binary_to_dec(literal),
            PacketData::Operator(sub_packets) => {
                match self.id.as_str() {
                    "000" => sub_packets.iter().map(|packet| packet.value()).sum(),
                    "001" => sub_packets.iter().map(|packet| packet.value()).fold(1, |acc,val| acc * val ),
                    "010" => sub_packets.iter().map(|packet| packet.value()).min().unwrap(),
                    "011" => sub_packets.iter().map(|packet| packet.value()).max().unwrap(),
                    "101" => {
                        let first = sub_packets.iter().next().unwrap().value();
                        let second = sub_packets.iter().nth(1).unwrap().value();
                        if first > second {
                            1
                        } else {
                            0
                        }
                    },
                    "110" => {
                        let first = sub_packets.iter().next().unwrap().value();
                        let second = sub_packets.iter().nth(1).unwrap().value();
                        if first < second {
                            1
                        } else {
                            0
                        }
                    },
                    "111" => {
                        let first = sub_packets.iter().next().unwrap().value();
                        let second = sub_packets.iter().nth(1).unwrap().value();
                        if first == second {
                            1
                        } else {
                            0
                        }
                    },
                    _ => panic!()
                }
            }
        }
    }
}


fn binary_to_dec(data: &String) -> u128 {
    let mut binary = data.chars().map(|car| car.to_digit(10).unwrap() as u128).collect::<Vec<_>>();

    binary.reverse();

    binary.into_iter().enumerate().map(|(index, data)|
        data * pow2(index)
    ).sum()
}

pub fn execute(input: &str) {
    let hex = input.split("\n").map(|v| v.to_string())
        .filter(|line| !line.is_empty())
        .next().unwrap();


    let binary = to_binary(hex);

    let mut packets:Vec<Packet> = vec![];

    let mut index = 0;
    let mut continu = true;

    while continu {
        let (packet, new_index) = decode(&binary, index);

        index = new_index;
        packets.push(packet);


        continu = binary[index..].chars().any(|car| car == '1');
    }

    let version_sum: u128 = packets.iter().map(|packet| packet.version()).sum();

    println!("Step 1 result {:?}", version_sum);

    let step2_result: u128 = packets.iter().map(|packet| packet.value()).sum();

    println!("Step 2 result {}", step2_result)


}

fn decode(binary: &String, mut index: usize) -> (Packet, usize) {
    let packet_version = binary[index..(index + 3)].to_string();
    let packet_id = binary[(index + 3)..(index + 6)].to_string();

    index = index + 6;

    let packet = if packet_id == "100".to_string() {

        let (packet_bits, new_index) = decode_literal(&binary, index);
        index = new_index;

        Packet {
            id: packet_id,
            version: packet_version,
            data: PacketData::Literal(packet_bits.join(""))
        }
    } else {
        let sub_type = binary.chars().nth(index).unwrap();

        index = index + 1;

        let size_size_field = if sub_type == '1' { 11 } else { 15 };

        let size = binary_to_dec(&binary[index..(index + size_size_field)].to_string()) as usize;

        index = index + size_size_field;

        let index_before_data = index;

        let mut data: Vec<Packet> = vec![];

        while sub_type == '0' && (index - index_before_data) < size || sub_type == '1' && data.len() < size {
            let (sub_packet, new_index) = decode(&binary, index);
            index = new_index;
            data.push(sub_packet);
        }

        Packet {
            id: packet_id,
            version: packet_version,
            data: PacketData::Operator(data)
        }
    };
    (packet, index)
}

fn decode_literal(binary: &String, mut index: usize) -> (Vec<String>, usize) {
    let mut packet_bits: Vec<String> = vec![];
    while binary.chars().nth(index).unwrap() == '1' {
        packet_bits.push(binary[(index + 1)..(index + 5)].to_string());
        index = index + 5;
    }
    packet_bits.push(binary[(index + 1)..(index + 5)].to_string());
    index = index + 5;
    (packet_bits, index)
}
