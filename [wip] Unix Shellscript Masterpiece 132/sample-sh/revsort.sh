#!/bin/sh

# 임시 파일 지정
tmpfile="sort.lst"

# 대상 ID 파일 확인
#(1)
if [ ! -f "$1" ]; then
  echo "ID 목록 파일을 지정하세요." >&2
  exit 1;
fi

# ID 끝 문자 숫자로 목록 정렬
rev "$1" | sort | rev > $tmpfile  #(2)

# 정렬한 ID 목록으로 리포트 작성
./report.sh $tmpfile  #(3)

# 임시 파일 삭제
rm -f $tmpfile
