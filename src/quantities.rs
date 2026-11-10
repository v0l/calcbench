pub struct Quantity {
    pub name: &'static str,
    pub units: &'static [&'static str],
}

pub const QUANTITIES: &[Quantity] = &[
    Quantity { name: "length", units: &["m", "mm", "cm", "km", "um", "nm", "in", "ft", "yd", "mi", "nmi", "mil", "angstrom"] },
    Quantity { name: "area", units: &["m^2", "cm^2", "mm^2", "km^2", "ha", "acre", "ft^2", "in^2", "yd^2", "mi^2"] },
    Quantity { name: "volume", units: &["L", "mL", "m^3", "cm^3", "gal", "qt", "pt", "cup", "floz", "ft^3", "in^3", "ukgal", "ukpt", "ukfloz"] },
    Quantity { name: "mass", units: &["kg", "g", "mg", "ug", "tonne", "lb", "oz", "stone", "carat", "grain"] },
    Quantity { name: "time", units: &["s", "ms", "us", "ns", "min", "h", "day", "week", "year"] },
    Quantity { name: "speed", units: &["m/s", "km/h", "mph", "knot", "ft/s", "cm/s"] },
    Quantity { name: "acceleration", units: &["m/s^2", "ft/s^2", "gee", "cm/s^2"] },
    Quantity { name: "force", units: &["N", "kN", "mN", "lbf", "kgf", "dyn", "ozf"] },
    Quantity { name: "pressure", units: &["Pa", "kPa", "MPa", "hPa", "bar", "mbar", "psi", "atm", "torr", "mmHg", "inHg", "inH2O"] },
    Quantity { name: "energy", units: &["J", "kJ", "MJ", "mJ", "Wh", "kWh", "cal", "kcal", "BTU", "eV", "erg", "ft_lbf"] },
    Quantity { name: "power", units: &["W", "mW", "uW", "kW", "MW", "hp", "BTU/h"] },
    Quantity { name: "temperature", units: &["C", "F", "K"] },
    Quantity { name: "angle", units: &["rad", "deg", "arcmin", "arcsec", "turn", "grad", "mrad"] },
    Quantity { name: "frequency", units: &["Hz", "kHz", "MHz", "GHz", "mHz", "rpm"] },
    Quantity { name: "data size", units: &["bit", "B", "kB", "MB", "GB", "TB", "KiB", "MiB", "GiB", "TiB"] },
    Quantity { name: "data rate", units: &["bit/s", "kbps", "Mbps", "Gbps", "B/s", "kB/s", "MB/s", "GB/s"] },
    Quantity { name: "current", units: &["A", "mA", "uA", "nA", "kA"] },
    Quantity { name: "voltage", units: &["V", "mV", "uV", "nV", "kV", "MV"] },
    Quantity { name: "resistance", units: &["ohm", "mΩ", "kΩ", "MΩ", "GΩ"] },
    Quantity { name: "capacitance", units: &["F", "mF", "uF", "nF", "pF"] },
    Quantity { name: "inductance", units: &["H", "mH", "uH", "nH"] },
    Quantity { name: "charge", units: &["coulomb", "mC", "uC", "nC", "Ah", "mAh"] },
    Quantity { name: "magnetic flux", units: &["Wb", "mWb", "uWb", "maxwell"] },
    Quantity { name: "magnetic field", units: &["T", "mT", "uT", "nT", "gauss", "mgauss"] },
    Quantity { name: "luminous flux", units: &["lm", "klm", "cd sr"] },
    Quantity { name: "illuminance", units: &["lux", "klux", "footcandle", "lm/m^2"] },
    Quantity { name: "luminance", units: &["cd/m^2", "nit", "kcd/m^2"] },
    Quantity { name: "radiation dose", units: &["Sv", "mSv", "uSv", "rem", "mrem"] },
    Quantity { name: "absorbed dose", units: &["Gy", "mGy", "uGy", "cGy"] },
    Quantity { name: "radioactivity", units: &["Bq", "kBq", "MBq", "GBq", "Ci", "mCi", "uCi"] },
    Quantity { name: "torque", units: &["N m", "kN m", "mN m", "kgf m", "ft lbf", "in lbf", "ozf in"] },
    Quantity { name: "flow", units: &["m^3/s", "L/s", "L/min", "mL/min", "m^3/h", "gpm", "cfm", "ft^3/s"] },
    Quantity { name: "density", units: &["kg/m^3", "g/cm^3", "g/mL", "kg/L", "lb/ft^3", "lb/in^3"] },
    Quantity { name: "dynamic viscosity", units: &["Pa s", "mPa s", "cP", "P"] },
    Quantity { name: "kinematic viscosity", units: &["m^2/s", "St", "cSt", "mm^2/s"] },
    Quantity { name: "fuel economy", units: &["mpg", "mi/ukgal", "km/L", "L/100km"] },
    Quantity { name: "molarity", units: &["mol/L", "mol/m^3", "mmol/L", "umol/L"] },
    Quantity { name: "mass concentration", units: &["kg/m^3", "g/L", "mg/L", "ug/L", "g/mL"] },
    Quantity { name: "fraction", units: &["ppm", "ppb", "%"] },
    Quantity { name: "power level", units: &["W", "mW", "uW", "dBm", "dBW"] },
    Quantity { name: "voltage level", units: &["V", "mV", "uV", "dBV", "dBmV", "dBuV", "dBu"] },
    Quantity { name: "gain", units: &["dB", "dBi", "dBd"] },
];

pub fn is_log(unit: &str) -> bool {
    let u = unit.to_lowercase();
    u.starts_with("db")
}

pub fn find(name: &str) -> Option<&'static Quantity> {
    QUANTITIES.iter().find(|q| q.name == name)
}
