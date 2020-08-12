Wyoming Uplift Subset for Incline - tet mesh version 

This was used for CO2 work with Raj
Mesh Design represents a 6 degree slope with a near 25 degree on the tail. 
Input surfaces elevations have 300m spacing, this mesh is near 30m spacing.
The challenge is a smooth Delaunay mesh with good Voronoi values along the incline.

Tet mesh for FEHM needed buffered layers below and above the target layer to protect voronoi volumes.
All Voronoi volumes for middle target layer are within an epsilon of 75000 (not boundary).

Test VoroCrust on target middle incline layer.
work directory : /project/meshing/GEO_Integration/work/wyo_uplift/tet_incline/VC

Source mesh:
/scratch/fwo/tam/CO2/stack_EVrsu_v1/work/tet_imt.inp

V1 RSU MESH using Rock Springs Uplift surface 
See 2008 Project pages https://meshing.lanl.gov/proj/WYCO2_2008/catalog.html

Mesh is 200000 long x 7000 wide with Z from 0 to 5350 meters. 
Middle Material 2 has 50m horiz spacing and 30m spacing same as stack_slope_v3 and 4a 

Source Weber surface (top):
EV RSU Model RSU3D082608 Weber surface
/scratch/fwo/tam/wy_2008/ev_RSU3D082608/rsu/horizon/Block3_Pw_xyz_m.inp
Exported from grid rsu/horizon/Block3_Pw.2grd (tam, 30 Sep 2008)
Grid_size: 94 x 86
Grid_space: 635454.545617,720000.000000,4585685.183704,4662500.000000 (feet)
EV surface size converted to meters:
 NAME              MIN               MAX         DIFFERENCE    LENGTH  
 xic     1.936865455E+05  2.194560000E+05 2.576945450E+04      8084  
 yic     1.397716844E+06  1.421130000E+06 2.341315601E+04      8084  
 zic    -4.745460370E+03  3.595548346E+02 5.105015205E+03      8084  

Note this surface has almost 300m spacing and extents to not extend far
enough south to include the full hump. This mesh front is at furthest south edge.
For larger domain and better resolution, use EV tools or contact John Jiao for data.

Tet Mesh
nodes =     1526607 
elements =  8798292


