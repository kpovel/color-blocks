select y, x, color
from blocks
         inner join available_colors ac on blocks.color_id = ac.id;
