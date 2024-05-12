#!/sbin/fish

cd ./shaders/ && for f in ./*; glslc $f -o ../compiled/$f.spv; end && cd ..
