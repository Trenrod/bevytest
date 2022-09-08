from http.client import TOO_MANY_REQUESTS
from PIL import Image

assets_dir = "D:/dev/game/assets/"

idle_images = Image.open(assets_dir + "idle.png")
moveing_images = Image.open(assets_dir + "chara1.png")
emote_images = Image.open(assets_dir + "emote1.png")

max_item_dimension = [0, 0]

def get_part(im, cols, rows, box):
    # box = (start_col, start_row, end_col, end_row)
    item_x = im.size[0] / cols
    item_y = im.size[1] / rows

    if max_item_dimension[0] < int(item_x):
        max_item_dimension[0] = int(item_x)
    if max_item_dimension[1] < int(item_y):
        max_item_dimension[1] = int(item_y)

    # left, upper, right, lower
    box = (box[0]*item_x, box[1]*item_y, box[2]*item_x, box[3]*item_y)
    return im.crop(box)

part_idle = get_part(idle_images, 12, 8, (0,0,3,1))
part_moveing = get_part(moveing_images, 12, 8, (0,0,3,4))
part_emote = get_part(emote_images, 12, 8, (0,0,3,3))

# Recreate partly images by reposition items
def recreate_by_reposition(im, cols, rows):
    new_im = Image.new("RGBA", (cols*max_item_dimension[0], rows*max_item_dimension[1]))

    item_x = im.size[0] / cols
    item_y = im.size[1] / rows

    offset_item_x = int((max_item_dimension[0] - item_x) / 2)
    # offset_item_y = int((max_item_dimension[1] - item_y) / 2)
    offset_item_y =int((max_item_dimension[1] - item_y))

    # For each item crop it and reposition it
    for col in range(0, cols):
        for row in range(0, rows):
            crop_box = (col*item_x, row*item_y, (col+1)*item_x, (row+1)*item_y)
            print(f"{crop_box}")
            part = im.crop(crop_box)

            paste_box = (col*max_item_dimension[0]+ offset_item_x, row*max_item_dimension[1] + offset_item_y)
            new_im.paste(part, paste_box)

    return new_im

part_idle = recreate_by_reposition(part_idle, 3, 1)
part_emote = recreate_by_reposition(part_emote, 3, 3)
part_moveing = recreate_by_reposition(part_moveing, 3, 4)

# Merge all they have the same widt
new_width = max(part_idle.size[0], part_moveing.size[0], part_emote.size[0])
new_height = part_idle.size[1] + part_moveing.size[1] + part_emote.size[1]
new_image = Image.new("RGBA", (new_width, new_height))


new_image.paste(part_idle, (0, 0))
new_image.paste(part_moveing, (0, part_idle.size[1]))
new_image.paste(part_emote, (0, part_idle.size[1] + part_moveing.size[1]))


def get_box(im, cols, rows, col, row):
    item_size_x = im.size[0]/cols
    item_size_y = im.size[1]/rows
    return (col*item_size_x, row*item_size_y, (col+1)*item_size_x, (row+1)*item_size_y)

new_image.save(assets_dir + "swordfighter.png")
