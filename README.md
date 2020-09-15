# LANL VoroCrust for Geologic Applicatons


This work integrates VoroCrust Meshing with Geologic Modeling Applications such as PFLOTRAN or Amanzi/ATS.


**VoroCrust**  Voronoi Meshing Without Clipping is the first provably correct algorithm for conforming Voronoi meshing for non-convex and possibly non-manifold domains with guarantees on the quality of both surface and volume elements. It provides a robust polyhedral meshing that can handle broad classes of domains exhibiting arbitrary curved boundaries and sharp features. In addition, the power of primal-dual mesh pairs, exemplified by Voronoi-Delaunay meshes is recognized as an important ingredient in numerous formulations. 



* [ VoroCrust Home vorocrust.sandia.gov ](https://vorocrust.sandia.gov/)
* [ Abstract ](https://arxiv.org/abs/1902.08767)
* [ PDF VoroCrust: Voronoi Meshing Without Clipping ](https://www.dropbox.com/s/qc6sot1gaujundy/VoroCrust.pdf)
* [ PDF VoroCrust Supplemental Materials ](https://www.dropbox.com/s/6p72h1e2ivw6kj3/VoroCrust_supplemental_materials.pdf)


Related Work:

* [Exodus Polyhedra Example Repo](https://github.com/daniellivingston/exodus-polyhedra)



# VoroCrust Instructions for LANL Models

**Usage:**
  vc [-h] <method> <input_filename>
 
**Methods:**
* -vc    : Use the vc method
* -viz   : Use the viz method
* -vcomp : Use the vcomp method
* -vlp   : Solve a Linear Program
* -covid : Analyze Covid-19 Data
* -h     : Display help message and exit.


Required Input Files: parameter file and surface file .obj

To run on command line: 
```
vorocrust -vc parameters.in
```

The parameter_file such as vc.in, has the input parameters for vorocrust. An example:

```
INPUT_MESH_FILE = ../surfmesh.obj
R_MAX = 10000.0
LIP_CONST = 0.25
VC_ANGLE = 30.0
NUM_THREADS = 10
GENERATE_VCG_FILE
```

VoroCrust will report to screen as it runs. Here is a sample portion of what you should see:

```
...
VoroCrust::Generating Explicit Voronoi Mesh:
  * Subregion 1/2
    -> Number of region seeds = 14900
  * Subregion 2/2
    -> Number of region seeds = 10067
  * executed in 2717.44 seconds!
VoroCrust::Saving VoroCrust Geomerty (.vcg) file:
  * Total Volume = 5.1975e+09

*** VoroCrust::Mission Accomplished in 3114.48 seconds! ***
```


## VoroCrust Input parameters:


See [ VoroCrust ](https://vorocrust.sandia.gov/) for VoroCrust Documentation.

See [ https://arxiv.org/abs/1902.08767 ](https://arxiv.org/abs/1902.08767) for details on VoroCrust parameters.


**INPUT_MESH_FILE** = filename.obj triangulated surface in .obj format
LaGriT can extract a valid 2D surface mesh from a 3D mesh and write .stl format files. Paraview can convert .stl files to .obj with read .stl then under File/Save Data write the .obj file. 


**R_MAX** = Maximum Sphere Radius is a bound on the sizing function and indicates the desired size of mesh elements. For most cases, set R_MAX to a very large value (e.g. the diagonal of the domain bounding box). Use R_MAX to set element size and enforce a finer mesh (based on physics not geometry). 
VoroCrust performs additional iterations to avoid having surface seeds.
Play with smaller R_MAX to have better results along boundary edges.


**LIP_CONST** = Lipschitz constant between 0 and 1, most cases will do well with a value of .25. Value 0 will force the sampling to be uniform, the smallest feature size will be applied everywhere (which may result in too many points). Value 1. will produce fewer points but is computationally more expensive and may not have a solution.


**VC_ANGLE**  = VC Smooth Angle Threshold in degrees usually between 20. and 80 and is discussed in paper. This angle threshold value is used to identify the sharp features and round aproximation errors.


**REF_ANGLE** = Refinement Smooth Angle Threshold in degrees with max at 180 degrees. 
This is used when the user decides to use the surface subdivision method as a preprocess. This would make the dihedral angles in the smooth patches go to 180. A value of 180 means that everything is smooth and the loop refinement well smear any sharp that exist.
This angle needs to be as high as possible to avoid false positives in sharp corner detection. For a stacked layer mesh with boundary poly lines at 50m, use Lip_Const .25 and REF_ANGLE 85, if REF_ANGLE is too small such as 40, the code may produce errors. 


**NUM_LOOP_REF** = Number of Refine Loops integer is a value 0 or greater. A value greater than 0 indicates the number of loop refinements, usually 6 or less for refinement.


**NUM_THREADS** specifies the number of threads that OpenMp uses

 
**GENERATE_VCG_FILE** will write a vcg file incuding volume and area information of the generated mesh


**NO_SHARP_CORNERS** = optional keyword



## VoroCrust Output Files:

***VoroCrustLog.txt*** - VoroCrust Report

***Voronoi_Seeds.cs*** - x-coordinate, y-coordinate, z-coordinate, sizing function, (3 components for the normal vector for surface seeds: nx, ny, nz), the following 6 number is additional attributes: number of additional attributes + 1,  index of pair seed (for surface seeds), indices of three spheres forming that seed, index of the subregion where that seed belong: (0 means a ghost seed that lie in the exterior of the domain).


***corner_spheres.csv***  ***edges_spheres.csv*** ***surface_spheres.csv*** - these are spheres at features showing the sphere packing with values x, y, z, and radius.


***mesh.vcg*** - voronoi CELL centers, volumes, and region ID followed by CONNECTIONS Cell_ID1, Cell_ID2, x_face, y_face, z_face,   face_area,  x_norm, y_norm,  z_norm.

Where *_face are coordinates on the line between the two cell centers where that line intersects the plane of the face. 
This is the not the center of the face, and may not even be on the face in some cases. (See the attached schematic)
*_norm are the components of the outward normal vector
 
The boundary nodes are also in the connections section.  They are flagged as connections with themselves.
For example:
4 4 0.1946231082256368 1 0.6037498441995969 0.000169276174283866 0 1 0
Cell 4 is a boundary. 
The normal from the center intersects the exterior face is (0.19, 1.0, 0.60),
face area is 1.7e-4 and the normal (0,1,0) is outward in the y direction.
 

## VIS of VoroCrust output:

***surface_mesh.obj*** - Surface Reconstruction of the volumetric mesh

***Vmesh_001.ply, Vmesh_002.ply***, for each region - these are the polygon faces of the mesh cells viewable with Paraview

***mesh.exo*** - not implemented yet, this is the polyhedral mesh viewable with Paraview

***clean.obj*** - is the input surface with duplicate nodes removed and possible issues fixed.


## Definitions

All 3D input files must be a triangulated watertight surface:

- no duplicate nodes
- all positive areas
- for a watertight mesh, each edge needs to appear in (at least) two facets.
- no holes in the surface, 0 boundary for a closed surface
- The triangulated surface mesh should be watertight and have a counter-clockwise orientation on all faces.
- For stacked internal boundaries, we are assigning normals up (top will be up, bottom faces will be down).

A manifold mesh has well defined volumes with the following properties:

- Every edge belongs to two faces.
- Every vertex is surrounded by one sequence of edges and faces.
- Faces only intersect eachother in common edges en vertexes.
- There is a material on only one side of a face

A non-manifold mesh might have one or more elements with the following properties:

- An edge incident to more than two faces.
- Two or more faces connected only by a vertex and not by an edge.
- Adjacent faces whose normals are pointing in opposite directions.
- The same geometry occupies the same space.



### Checks for input surfaces:

Paraview can be used to check normal orientation of faces.
use the filter "Normal Glyph" filter, it will draw arrow glyphs pointing in the normal directions.


Paraview "Backface" corresponds to the faces that have their normal pointing in same direction than the viewpoint.
Such faces are not visible if object is opaque, the normals are correctly computed and if you look the object from the exterior. There are options to specify the color for such faces and their mode of representation.


At LANL, LaGriT is used to extract the external surface from single or multi-material meshes.
The following LaGriT tools are used to check for good surface quality.
 [See more on LaGriT Commands ](http://lanl.github.io/LaGriT/pages/commands.html)


There are no duplicate nodes
```
   LaGrit command: filter/1,0,0
```

Check connetivity. For a single material surface geniee will report no more than 2 jtet loops.
For multi-material there will be more than 2 loops with no detirministic solution.
```
LaGriT command: geniee
LaGriT sample good output:
  geniee: mesh has   4  jtet loops, max cycle length =  3
```

All faces have a positive area.
```
LaGriT command: quality
LaGriT sample good output:
  All elements have volume  5.0000000E-01
```

All elements are connected with no openings or holes.
```
LaGriT command: boundary_components:
LaGriT sample good output:
  0  different exterior boundary components
```

Check Topology.
When you run geniee if it is not in 2 facets, it is type ibndry=16,000,000
For a single material surface, this command can be used to flip orientation.
```
LaGriT command: geniee mos 2dnormal 0
LaGriT output:
 ---TOPOLOGY MODIFY----
  Number of Elements Total           22
  Number of Elements Tested          22
  Reference Element                   1
  Inconsistent Elements               0
 ---NO ACTION, Only Check---
```

Check normals.
LaGriT command: settets/normal
LaGriT output is a mesh with faces colored in 6 directions.
Each face is colored in one of 6 directions with all facing outward
With top facing up, bottom facing down, etc including right, back, left, front.


## Information Needed for Model Simulations

1. Sparse matrix graph is edge graph of the Delaunay dual of the Voronoi tessellation. It is the connectivity graph of the Voronoi tessellation describing how the faces of the volumes are connected.
2. Volume of each Voronoi cell - to fill the Diagonal entries of the sparse matrix
3. Areas of each Voronoi cell face - to fill the off diagonal entries of the sparse matrix where area_ij = area_ji so only half of the matrix is written.
4. Material id for each cell if multi-material.
5. List or defined regions to represent faces for flow or no-flow boundaries of the mesh.
6. Exodus Polyhedral Mesh format
7. Statistics and VIS files to check and document

