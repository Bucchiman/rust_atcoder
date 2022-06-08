#!/bin/zsh
#
# FileName: 	change_src
# CreatedDate:  2022-06-07 23:28:02 +0900
# LastModified: 2022-06-08 16:40:00 +0900
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

echo "use proconio::input;\n" > tmp.rs
cat $object_file | awk -v flag=0 '{
    if ($1 == "pub" || flag == 1){
        if (flag == 0){
            flag = 1
            printf "fn main() {\n"
        }
        else{
            print $0
        }
    }
}' >> tmp.rs

return
