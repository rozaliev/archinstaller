
qemu-system-x86_64 --bios /usr/share/OVMF/OVMF_CODE.fd -enable-kvm -machine q35 -device intel-iommu -cpu host -drive file=arch-hdd.img,format=raw -m 4G -netdev user,id=n0,smb=/home/erz/Documents/scripts -nic user,id=n0
