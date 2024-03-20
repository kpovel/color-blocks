select y, x, color
from blocks
         inner join available_colors ac on blocks.color_id = ac.id;

update blocks
set color_id = (select id from available_colors where color = :color)
where y = :y
  and x = :x;
