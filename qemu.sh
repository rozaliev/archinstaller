
qemu-system-x86_64 -enable-kvm -machine q35 -device intel-iommu -cpu host -drive file=arch-hdd.img,format=raw -m 4G -netdev user,id=n0,smb=/home/erz/Documents/scripts -nic user,id=n0
