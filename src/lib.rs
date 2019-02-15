use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let info = Info::new(&config.filename)?;

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub snapnum: u32,
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Incorrect arguments: [SNAPNUM] [QUERY]");
        }

        let snapnum = match args[1].parse::<u32>() {
            Ok(num) => num,
            Err(err) => return Err("[SNAPNUM] must be integer!")
        };
        let mut filename = String::from("output_");
        let suffix = format!("{:05}/info_{:05}.txt", snapnum, snapnum);
        filename.push_str(&suffix);
        let query   = args[2].clone();

        Ok(Config { snapnum, filename, query })
    }
}

#[derive(Debug)]
struct Info {
    ncpu: u32,
    ndim: u32,
    levelmin: u32,
    levelmax: u32,
    ngridmax: u32,
    nstep_coarse: u32,
    boxlen: f64,
    time: f64,
    H0: f64,
    omega_m: f64,
    omega_l: f64,
    omega_k: f64,
    omega_b: f64,
    unit_l: f64,
    unit_d: f64,
    unit_t: f64,
    redshift: f64,
    smallh: f64,
    boxsize: f64,
    Hz: f64,
}

impl Info {
    fn new(filename: &String) -> Result<Info, Box<dyn Error>> {
        let contents   = fs::read_to_string(&filename)?;
        let mut buffer = Vec::new();

        for line in contents.lines() {
            buffer.push(line);
        }
        println!("Opened: {}", &filename);

        let ncpu: u32         = Info::extract(buffer[0]).parse()?;
        let ndim: u32         = Info::extract(buffer[1]).parse()?;
        let levelmin: u32     = Info::extract(buffer[2]).parse()?;
        let levelmax: u32     = Info::extract(buffer[3]).parse()?;
        let ngridmax: u32     = Info::extract(buffer[4]).parse()?;
        let nstep_coarse: u32 = Info::extract(buffer[5]).parse()?;

        let boxlen: f64   = Info::extract(buffer[7]).parse()?;
        let time: f64     = Info::extract(buffer[8]).parse()?;
        let aexp: f64     = Info::extract(buffer[9]).parse()?;
        let H0: f64       = Info::extract(buffer[10]).parse()?;
        let omega_m: f64  = Info::extract(buffer[11]).parse()?;
        let omega_l: f64  = Info::extract(buffer[12]).parse()?;
        let omega_k: f64  = Info::extract(buffer[13]).parse()?;
        let omega_b: f64  = Info::extract(buffer[14]).parse()?;
        let unit_l: f64   = Info::extract(buffer[15]).parse()?;
        let unit_d: f64   = Info::extract(buffer[16]).parse()?;
        let unit_t: f64   = Info::extract(buffer[17]).parse()?;
        
        let redshift = Info::find_redshift(aexp);
        let smallh   = H0/100.0;
        let boxsize  = Info::find_boxsize(unit_l, boxlen);
        let Hz       = Info::find_Hz(aexp, H0, omega_m, omega_k, omega_l);

        Ok(Info { ncpu,
                  ndim,
                  levelmin,
                  levelmax,
                  ngridmax,
                  nstep_coarse,
                  boxlen,
                  time,
                  H0,
                  omega_m,
                  omega_l,
                  omega_k,
                  omega_b,
                  unit_l,
                  unit_d,
                  unit_t,
                  redshift,
                  smallh,
                  boxsize,
                  Hz, }
          )
    }

    fn extract(buf: &str) -> &str {
        &buf[14..].trim()
    }

    fn find_redshift(aexp: f64) -> f64 {
        1.0/aexp - 1.0
    }

    fn find_boxsize(unit_l: f64, boxlen: f64) -> f64 {
        boxlen * unit_l / (3.08 * 10.0f64.powf(24.0))
    }

    fn find_Hz(aexp: f64, H0: f64, omega_m: f64, omega_k: f64, omega_l: f64) -> f64 {
        let Ez = omega_m / aexp.powf(3.0)
            + omega_k / aexp.powf(2.0)
            + omega_l;
        H0 * Ez.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_extract() {
        let test_string = String::from("boxlen      =  0.100000000000000E+01");
        assert_eq!(Info::extract(&test_string), " 0.100000000000000E+01");
    }
            
}
