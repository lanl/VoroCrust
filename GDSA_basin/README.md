# GDSA Basin Example from JewelSuite 2020


This version V03 has volumes that are too large and not uniform, resulting in too much dispersion as compared to the LaGriT uniform tet mesh.

VoroCrust vc.in for V03 used in PFLOTRAN Simulations for Report.
```
INPUT_MESH_FILE = ../surfmesh_flat_xalign.obj
R_MAX = 100.0
LIP_CONST = 0.25
VC_ANGLE = 60.0
GENERATE_VCG_FILE 
```

Versions to create more uniform and smaller volumes for PFLOTRAN simulations.
```
Vers        Parameters     Cells    Min Volume	  Max Volume
LaGrit Tet  50x20	         103950   6250	        50000
1	R=10000   LIP=.25        15751    1872	        3610362
2	R=50      LIP=.25        76000    180	          1203425
3	R=100     LIP=.25        24967    1408	        2486226
4	R=50      LIP=.15        108134   153	          760714
7	R=25      LIP=.15        318343   22	          269455
```


V01_R10000_L25

<img width="500" src="images/vc_V01_R10000_L25_clip.png"> 


V02_R50_L25

<img width="500" src="images/vc_V02_R50_L25_clip.png"> 


V03_R100_L25 (PFLOTRAN Simulations)

<img width="500" src="images/vc_V03_R100_L25_clip.png"> 


V04_R50_L15

<img width="500" src="images/vc_V04_R50_L15_clip.png"> 


V05_R60_L15

<img width="500" src="images/vc_V05_R60_L15_clip.png"> 


V06_R60_L25

<img width="500" src="images/vc_V06_R60_L25_clip.png">


V07_R25_L15

<img width="500" src="images/vc_V07_R25_L15_clip.png"> 


