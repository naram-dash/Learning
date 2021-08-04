#!/bin/sh

# CSV파일 지정
csvfile="data.csv"

# ID가 지정되지 않으면 종료
#(1)
if [ -z "$1" ]; then
  echo "ID를 지정하세요." >&2
  exit 1
fi

# CSV 파일이 존재하지 않으면 종료
#(2)
if [ ! -f "$csvfile" ]; then
  echo "CSV 파일이 존재하지 않습니다: $csvfile" >&2
  exit 1
fi

#(3)
while read line
do
  # cut으로 컬럼 추출
  #(4)
  id=$(echo $line | cut -f 1 -d ',')
  name=$(echo $line | cut -f 2 -d ',')
  score=$(echo $line | cut -f 3 -d ',')

  # ID 컬럼이 인수로 지정한 ID와 일치하면 표시
  #(6)
  if [ "$1" = "$id" ]; then
    echo "$name"
  fi
done < $csvfile  #(5)
