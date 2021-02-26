cargo bootimage --target target.json
if [ $? == 0 ]; then
	qemu-system-x86_64 /media/HDD/Workfolder/RUST/RheydOS/target/target/debug/bootimage-nonamekernel.bin
else
	echo "Error"
fi
