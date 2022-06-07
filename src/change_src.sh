#!/bin/zsh
#
# FileName: 	change_src
# CreatedDate:  2022-06-07 23:28:02 +0900
# LastModified: 2022-06-07 23:51:03 +0900
#


_usage(){
    echo "Usage: $0 -f object_file"
    exit 1
}

if [ -e tmp.rs ];
then
    rm tmp.rs
fi

while getopts :f: OPT
do
    case $OPT in
        f) object_file=$OPTARG;;
        :|\?) _usage;;
    esac
done

echo "use procon::input;\n" > tmp.rs
cat $object_file | awk -v flag=0 '{
    if ($1 == "pub" || flag == 1){
        print $0
        flag = 1
    }
}' >> tmp.rs

return
