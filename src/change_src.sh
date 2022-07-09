#!/bin/zsh
#
# FileName: 	change_src
# CreatedDate:  2022-06-07 23:28:02 +0900
# LastModified: 2022-07-09 09:46:41 +0900
#


_usage(){
    echo "Usage: $0 -f object_file"
    exit 1
}

if [ -e tmp.rs ];
then
    rm tmp.rs
fi

while getopts :f:n: OPT
do
    case $OPT in
        f) object_file=$OPTARG;;
        n) name_func=$OPTARG;;
        :|\?) _usage;;
    esac
done

echo "use proconio::input;\n" > tmp.rs
cat $object_file | awk -v flag=0 -v fn="${name_func}" '
{
if ($3 == fn || $3 == fn "()" || flag == 1){
        if (flag == 0){
            flag = 1;
            printf "fn main() {\n";
        }
        else{
            print $0;
        }
    }
if (flag == 1 && length($0) == 1 && $1 == "}"){
    flag = 0;
}
}' >> tmp.rs

return
