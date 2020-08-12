# LANL VoroCrust for Geologic Applicatons


This work integrates VoroCrust Meshing with our Geologic Modeling Applications.


**VoroCrust**  Voronoi Meshing Without Clipping is the first provably correct algorithm for conforming Voronoi meshing for non-convex and possibly non-manifold domains with guarantees on the quality of both surface and volume elements. It provides a robust polyhedral meshing that can handle broad classes of domains exhibiting arbitrary curved boundaries and sharp features. In addition, the power of primal-dual mesh pairs, exemplified by Voronoi-Delaunay meshes is recognized as an important ingredient in numerous formulations. 



* [ VoroCrust Home vorocrust.sandia.gov ](https://vorocrust.sandia.gov/)
* [ Abstract ](https://arxiv.org/abs/1902.08767)
* [ PDF VoroCrust: Voronoi Meshing Without Clipping ](https://www.dropbox.com/s/qc6sot1gaujundy/VoroCrust.pdf)
* [ PDF VoroCrust Supplemental Materials ](https://www.dropbox.com/s/6p72h1e2ivw6kj3/VoroCrust_supplemental_materials.pdf)



# VoroCrust Instructions for LANL Models


Required: parameter file and surface file .obj

To run on command line: 
```
vorocrust parameter_file_name
```

The parameter_file such as vc.in, has the input parameters for vorocrust. An example:

```
INPUT_MESH_FILE = ../surfmesh_gfm.obj
NUM_LOOP_REF = 0
REF_ANGLE = 180.0
R_MAX = 2. 
LIP_CONST = 0.25
VC_ANGLE = 80.0
```

VoroCrust will report to screen as it runs. Here is a sample portion of what you should see:

*The Min. dihedral angle in this report should be as close as possible to 180 (Values above 150 are usually fine).
If you see something like 120 when you specify a smoothness threshold angle of 60 degrees that is an indication that 
a very very fine mesh will be produced in a very long time.*

```
VoroCrust::Input Data:
           * Input Mesh = ../surfmesh_gfm.obj
           * Number of Loop Refinements = 0
           * Refinement Smooth Angle Threshold = 180
           * Maximum Sphere Radius = 2
           * Lipschitz Const = 0.25
           * VC Smooth Angle Threshold = 80
           * Input model is a watertight manifold.
VoroCrust::Reading obj file:
           * Min. dihedral angle between smooth neighbors = 180 degrees
           * Number of Input mesh points = 761
           * Number of Input mesh faces = 1632
           * Number of Sharp Corners = 92
           * Number of Sharp Edges = 274
VoroCrust::Generating Surface Seeds:
           * Number of Spheres = (92, 291, 1000)
           * Number of Spheres = (92, 291, 2000)
...
           * Number of ghost Volume seeds = 71376
           * executed in 154.842 seconds!
           * Saving Output in Interior_seeds.csv, Exterior_seeds.csv and Volume_seeds.csv

*** VoroCrust::Mission Accomplished! ***.
```

VoroCrust with NON_MANIFOLD will not report the Min. dihedral angle and will look similar to this.

*If you forget to add the NON_MANIFOLD keyword, there is is good chance the mesh will finish and may look reasonable. But using the NON_MANIFOLD keyword will make a better mesh. VoroCrust does not complain if you forget*

```
VoroCrust::Input Data:
           * Input Mesh = ../surfmesh_gfm.obj
           * Number of Loop Refinements = 0
           * Refinement Smooth Angle Threshold = 180
           * Maximum Sphere Radius = 2
           * Lipschitz Const = 0.25
           * VC Smooth Angle Threshold = 80
           * Input model is NOT watertight or NON manifold!
VoroCrust::Reading obj file:
           * Number of Input mesh points = 761
           * Number of Input mesh faces = 1632
...
           * executed in 102.396 seconds!
           * Saving Output in Interior_seeds.csv, Exterior_seeds.csv and Volume_seeds.csv

*** VoroCrust::Mission Accomplished! ***.
```


## VoroCrust Input parameters:


See [ https://arxiv.org/abs/1902.08767 ](https://arxiv.org/abs/1902.08767) for details on VoroCrust parameters.


**INPUT_MESH_FILE** = filename.obj triangulated surface in .obj format
LaGriT can extract a valid 2D surface mesh from a 3D mesh and write .stl format files. Paraview can convert .stl files to .obj with read .stl then under File/Save Data write the .obj file. 



**R_MAX** = Maximum Sphere Radius is a bound on the sizing function and indicates the desired size of mesh elements. For most cases, set R_MAX to a very large value (e.g. the diagonal of the domain bounding box). Use R_MAX to set element size and enforce a finer mesh (based on physics not geometry). 


**LIP_CONST** = Lipschitz constant between 0 and 1, most cases will do well with a value of .25. Value 0 will force the sampling to be uniform, the smallest feature size will be applied everywhere (which may result in too many points). Value 1. will produce fewer points but is computationally more expensive and may not have a solution.


**VC_ANGLE**  = VC Smooth Angle Threshold in degrees usually between 20. and 80 and is discussed in paper. This angle threshold value is used to identify the sharp features and round aproximation errors.



**REF_ANGLE** = Refinement Smooth Angle Threshold in degrees with max at 180 degrees. 
This is used when the user decides to use the surface subdivision method as a preprocess. This would make the dihedral angles in the smooth patches go to 180. A value of 180 means that everything is smooth and the loop refinement well smear any sharp that exist.
This angle needs to be as high as possible to avoid false positives in sharp corner detection. For a stacked layer mesh with boundary poly lines at 50m, use Lip_Const .25 and REF_ANGLE 85, if REF_ANGLE is too small such as 40, the code may produce errors. 



**NUM_LOOP_REF** = Number of Refine Loops integer is a value 0 or greater. A value greater than 0 indicates the number of loop refinements, usually 6 or less for refinement.


**NO_SHARP_CORNERS** = optional keyword


**NON_MANIFOLD** and/or **NON_WATERTIGHT** = conditional keywords needed if the input mesh is non-manifold or non-watertight. The prescence of either keyword will set water_tight_manifold = false.


## VoroCrust Output Files:


*surface_mesh.obj* - Surface Reconstruction of the volumetric mesh.


*Exterior_seeds.csv*  *Interior_seeds.csv* - are the Voronoi surface seeds outside and inside the boundary surface. 


*Interior_volume_seeds.csv* - these are the seeds used to fill the interior of the mesh domain.


*Exterior_volume_seeds.csv* - these seeds are the outer domain used to ensure that the aspect ratio of the generated Voronoi tessellation is consistent with the input value of the Lipschitz continuity constant.


*corner_spheres.csv*  *edges_spheres.csv* *surface_spheres.csv* - these are spheres at features showing the sphere packing with values x, y, z, and radius.


*corner_point_cloud.csv*  *edge_point_cloud.csv*  *surface_point_cloud.csv* - are used to speed up functions, can be ignored.


*Ghost_volume_seeds.csv* - is a set of seeds mirroring the point set outside the bounding box in 6 directions. It is used to demonstrate that VoroCrust can handle domains with multiple materials and can be ignored.




## VIS of VoroCrust output:

Voro++

Contact Ahmed Mahmoud ahmahmoud@ucdavis.edu

PhD student at ECE, UCDavis


http://math.lbl.gov/voro++/


To generate the explicit mesh (for a manifold model) you need to combine all interior and exterior seeds into one file, pass it to Voro++ and generate the cells associated with the interior seeds.
For non-manifold models (the definition of interior and exterior is now ambiguous and you need to mark each seed according to the region it belongs to).


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


## Under Development

1. Sparse matrix graph is edge graph of the Delaunay dual of the Voronoi tessellation. It is the connectivity graph of the Voronoi tessellation describing how the faces of the volumes are connected.
2. Volume of each Voronoi cell - to fill the Diagonal entries of the sparse matrix
3. Areas of each Voronoi cell face - to fill the off diagonal entries of the sparse matrix where area_ij = area_ji so only half of the matrix is written.
4. Material id for each cell if multi-material.
5. List or defined regions to represent faces for flow or no-flow boundaries of the mesh.
6. VIS and statistics files to check and docum
