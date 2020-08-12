Connect points into a Delaunay Mesh and Write FEHM style Files
Local work: /project/meshing/GEO_Integration/examples_lanl/2Cubes_2mat/tet_for_fehm

For details on the FEHM modeling files written by LaGriT
https://lanl.github.io/LaGriT/pages/docs/commands/dump/DUMP3.html

FEHM Modeling Files:

tet_full.stor - Sparse Matrix Voronoi Coefficients for FEHM
tet.stor - Sparse Matrix Voronoi Coefficients for FEHM, compressed version 
tet.fehmn - FEHM node and element mesh file
tet_outside.zone - FEHM zones listing nodes on each of 6 boundaries (bottom, top, front, right, back, left) 
tet_outside_vor.area - list of areas associated with the boundary nodes
tet_material.zone - FEHM zones listing nodes in each of the materials
tet_multi_mat.zone - list of node pairs on either side of an interface boundary

Additional Files:

README.txt

surf_cube_1bndry.inp - triangulated surface in AVS format
tri_node_attr.inp - triangulated surface with added attributes
tri_node_attr.dat - triangulated surface node attribute list
tet_connect_fehm.lgi - lagrit command file to connect surface points into a Delaunay tet mesh

tet.gmv, tet.inp - tet mesh in GMV and AVS format
tet_cell_attr.dat, tet_node_attr.dat - tet node and cell attribute lists





