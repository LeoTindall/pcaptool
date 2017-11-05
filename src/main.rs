extern crate pcap;
use pcap::Device;

#[macro_use] extern crate prettytable;
use prettytable::{Table, format};

fn main() {
    let devices = Device::list().expect("Could not list devices");

    let format = format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Title, format::LinePosition::Bottom],
            format::LineSeparator::new('─', '┼', '├', '┤')
        )
        .padding(1, 1)
        .build();

    let mut table = Table::new();
    table.set_format(format); 
    table.set_titles(row![b->"Device Name", b->"Description"]);

    for device in devices {
        let desc = match device.desc {
            Some(desc) => desc,
            None => "No description.".into(),
        };
        table.add_row(row![device.name, desc]);
    }

    table.printstd();

}
