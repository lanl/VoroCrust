/*
 *  MESH CONVERSION FROM VOROCRUST CSV TO AVS-UCD
 *
 * To compile,
 *    rustc to_avs.rs -o to_avs
 *    ./to_avs
 *
 * Running without arguments will print the help screen.
 *
 */

use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::io;
use std::ffi::OsStr;

enum Element {
    Point,
    Tri,
    Tet,
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

// TODO: add elements, attributes
struct Mesh {
    nodes: Vec<Point>,
    geom: Element,
}

// Converts a PathBuf to a string.
// Note that the caller will lose pointer ownership,
// unless the PathBuf is cloned first.
fn path_to_string(path: PathBuf) -> String {
    return path.into_os_string()
               .into_string()
               .unwrap();
}

// Parses a VOROCRUST *.csv file into a Mesh object.
fn read_mesh_from_file(input: PathBuf) -> Mesh {
    let data = fs::read_to_string(input).expect("Unable to read file");
    let lines: Vec<&str> = data.split('\n').collect();

    let mut points: Vec<Point> = Vec::new();

    // Iterate over each line in the file
    // Begin at i = 1 to ignore the header
    for i in 1..lines.len() {
        let v: Vec<&str> = lines[i].split(',').collect();
        
        // Ignore empty lines
        if v[0].trim() == "" { continue; }

        let p = Point {
            x: v[0].trim().parse().unwrap(),
            y: v[1].trim().parse().unwrap(),
            z: v[2].trim().parse().unwrap(),
        };

        points.push(p);
    }
    
    // Construct the mesh
    let input_mesh = Mesh {
        nodes: points,
        geom: Element::Point,
    };

    return input_mesh;
}

// Writes an AVS-UCD file from a Mesh instance.
fn write_mesh(mesh: Mesh, outfile: PathBuf) -> std::io::Result<()> {

    let num_nodes = mesh.nodes.len();
    let mut data = String::from("");

    // Write header
    data.push_str(&format!("{0} {0} 0 0 0\n",num_nodes));

    // Write nodes
    for i in 0..num_nodes {
        data.push_str(&format!("{:06} {:018.10} {:018.10} {:018.10}\n",
            (i+1),
            mesh.nodes[i].x,
            mesh.nodes[i].y,
            mesh.nodes[i].z)
        );
    }

    // Write elements
    // TODO: allow different element types to be used
    for i in 0..num_nodes {
        data.push_str(&format!("{:06} 1 pt {:06}\n",
            (i+1),
            (i+1))
        );
    }

    fs::write(outfile,data).expect("Unable to write file");
    Ok(())
}


// Finds all .csv files within a given directory.
fn list_of_csv_paths(root: &str) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];

    for path in fs::read_dir(root)? {
        let path = path?.path();
        if let Some("csv") = path.extension().and_then(OsStr::to_str) {
            result.push(path.to_owned());
        }
    }
    Ok(result)
}


fn parse_from_file(input: PathBuf, outdir: PathBuf) -> std::io::Result<()> {

    let mut files = vec![];

    // Validate input and output paths
    if input.exists() {
        if input.is_dir() {
            files = list_of_csv_paths(&path_to_string(input.clone())).unwrap();
        } else {
            files.push(input);
        }
    } else {
        panic!("Input path is not valid");
    }

    if outdir.exists() {
        if !outdir.is_dir() {
            panic!("Output directory exists - and is a file");
        }
    } else {
        fs::create_dir_all(&outdir).expect("Failed to create directory");
    }


    // Begin conversion process
    for path in files {

        // Create outfile path
        let filestem = path.file_stem().and_then(OsStr::to_str);
        let outfile  = Path::new(outdir.to_str().unwrap())
                       .join(filestem.unwrap())
                       .with_extension("inp");

        // Read infile
        println!("Reading {:?}",path);
        let input_mesh = read_mesh_from_file(path.clone());
        println!("=> found {} nodes",input_mesh.nodes.len());

        // Write
        println!("\nWriting to: {:?}",outfile);
        write_mesh(input_mesh,outfile.clone());

    }

    Ok(())

}

// TODO: add -from and -to flags
fn help() {
    println!("
Converts VOROCRUST *.csv output to AVS-UCD meshes.
Note that the delimeter must be `,`, though this can be changed by
editing the source.

USAGE
    input
        May be a directory or a file. If directory, a 
        conversion on all *.csv files will be attempted.
    [output_dir]
        Optional. If specified, files will be written to
        this directory.

EXAMPLES
    to_avs examples/seven_layers .
    to_avs examples/seven_layers output/
    to_avs examples/seven_layers/edge_spheres.csv .
    to_avs examples/seven_layers/edge_spheres.csv output/
");
}

fn main() -> std::io::Result<()> {
    // Capture and iterate over command-line arguments
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            help();
        },
        2 => {
            parse_from_file(PathBuf::from(&args[1]),env::current_dir().unwrap());
        },
        3 => {
            parse_from_file(PathBuf::from(&args[1]),PathBuf::from(&args[2]));
        },
        _ => {
            help();
        }
    }

    Ok(())
}

