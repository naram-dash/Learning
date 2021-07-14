module Geometry.Sphere
(volume
, area
) where

-- Geometry 폴더 밑의 Sphere.hs 파일이다.

volume :: Float -> Float  
volume radius = (4.0 / 3.0) * pi * (radius ^ 3)  
  
area :: Float -> Float  
area radius = 4 * pi * (radius ^ 2)  