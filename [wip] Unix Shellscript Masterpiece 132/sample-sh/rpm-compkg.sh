#!/bin/sh

# 파일을 지정하는 명령행 인수를 확인
#(1)
if [ ! -f "$1" ]; then
  echo "파일이 없습니다: $1" >&2
  exit 2
fi

# 파일명에서 속한 RPM 패키지명을 취득
#(2)
pkgname=$(rpm -qf "$1")

# rpm -qf 명령어 결과로 패키지명 표시
#(3)
if [ $? -eq 0 ]; then
  echo "$1 -> $pkgname"
else
  echo "$1은 패키지에 포함되지 않습니다." >&2
exit 1
fi
