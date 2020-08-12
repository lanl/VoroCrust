Original work directory:
/project/meshing/JewelSuite/liz_example_tests/Test_Case_01/Full

The file tet_materials.inp has the current final mesh.
Z is Elevation and tet elements have correct orientation.
Mesh is visually good, but still need to confirm with respect to JS input data.

Material numbers are not a direct correlation and will have to figure this out.
There are material id written to CompartmentId and ZoneId
We use CompartmentId for now.
Node imt and itetclr are corrected to JS material ids

JS unit 1 = top, mesh = 4
JS unit 2 = next, mesh = 2
JS unit 3 = next, mesh = 1
JS unit 4 = bot, mesh = 3



####### Convert from Z Down to Z Elevation #################
DO NOT FORGET to dos2unix the gocad mesh file!

head -50 input.so > gocad_header.txt

# get property names and minmax from gocad_header.txt

# mult Z by -1 (inverts tets)
# re-order tet nodes for positive volumes (1234 to 1324
#
# write tet connectivity and color
# The last property is most likely the Materials
# 1      2  3  4  5         6         7         8
# TETRA n1 n2 n3 n4   CompartmentId ElementId  ZoneId
# both CompartmentId and ZoneId have material numbers
# Use CompartmentId for now

awk '{if ($1 == "PVRTX") print $2,$3,$4, ($5*-1.)  }' input.so > nodes_minus.txt
awk '{ if ($1 == "TETRA") print $7,$8," tet ",$2,$4,$3,$5 }' input.so > elems_1324.txt

# make a header avsheader.txt
  957  4201 0 0 0

cat avsheader.txt nodes_minus.txt elems_1324.txt > tet_1324.inp

# copy good mesh into final file
lagrit < tet_materials.lgi 

############# INCORRECT COORD SYSTEM #################

# write mesh nodes (ignore $6 NodeId)
awk '{if ($1 == "PVRTX") print $2,$3,$4,$5 }' input.so > nodes.txt

# write tet connectivity and color
# The last property is most likely the Materials
# 1      2  3  4  5         6         7         8
# TETRA n1 n2 n3 n4   CompartmentId ElementId  ZoneId
awk '{ if ($1 == "TETRA") print $7,$8," tet ",$2,$3,$4,$5 }' input.so > elems_ZoneId.txt
awk '{ if ($1 == "TETRA") print $7,$6," tet ",$2,$3,$4,$5 }' input.so > elems_CompartmentId.txt

cat nodes.txt elems_ZoneId.txt > tet_ZoneId.inp
diff elems_CompartmentId.txt elems_ZoneId.txt


