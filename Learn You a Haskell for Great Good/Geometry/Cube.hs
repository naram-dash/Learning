module Geometry.Cube  
( volume  
, area  
) where  
  
import qualified Geometry.Cuboid as Cuboid  
-- import는 module 다음에 적는다.
  
volume :: Float -> Float  
volume side = Cuboid.volume side side side  
  
area :: Float -> Float  
area side = Cuboid.area side side side  