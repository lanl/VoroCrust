# loop through each directory
# /project/eesdev/tam/LaGriT/src/xlagrit < vcg_totet_quality.lgi

for dir in vc_params*; do  cd "$dir"; 
 echo cd $dir; 
 echo `pwd` ; 
 cp ../scripts/vcg_totet_quality.lgi .
 /project/eesdev/tam/LaGriT/src/xlagrit < vcg_totet_quality.lgi
 cp lagrit.out vcg_totet_quality.out.txt 
 cd ..; done;

