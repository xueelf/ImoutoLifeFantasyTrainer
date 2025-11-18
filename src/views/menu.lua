local config = require('config')
local pointer = require('pointer')
local util = require('util')

MainMenu = createMainMenu(Window)

-- Separator
local SeparatorMenu = createMenuItem(MainMenu)
SeparatorMenu.caption = '-'
-- Item
local ItemMenu = createMenuItem(MainMenu)
ItemMenu.caption = '道具(&I)'
-- Help
local HelpMenu = createMenuItem(MainMenu)
HelpMenu.caption = '帮助(&H)'

local ABOUT_TEXT = string.format([[Version %s

Powered by Cheat Engine
Copyright © 2025 by Yuki<admin@yuki.sh>]], config.version)
local DOCUMENTATION_URL = 'https://blog.yuki.sh/posts/05064242ec86/'

local item_sort = {
    'drinks',
    'presents',
    'books',
    'household_items',
    'sex_items'
}
local sub_item_sort = {
    drinks = {
        'green_tea',
        'coffee',
        'black_tea',
        'sleep_tea',
        'stimulant',
        'love_potion',
        'after_pill',
    },
    presents = {
        'sweets',
        'present',
    },
    books = {
        'adventure_book',
        'naughty_book',
        'doujinshi',
    },
    household_items = {
        'soft_pillow',
        'feather_mattress',
        'dumbbell',
        'iron_sandals',
    },
    sex_items = {
        'condom',
        'vibrator',
        'dildo',
        'anal_dildo',
    },
}
local item_caption_map = {
    drinks = '茶水类(&D)',
    green_tea = '绿茶',
    coffee = '咖啡',
    black_tea = '红茶',
    sleep_tea = '安眠茶',
    stimulant = '壮阳茶',
    love_potion = '媚药',
    after_pill = '避孕药',

    presents = '礼物类(&P)',
    sweets = '零食',
    present = '礼物',

    books = '书籍类(&B)',
    adventure_book = '冒险的书',
    naughty_book = '色色的书',
    doujinshi = '小薄本',

    household_items = '家具类(&H)',
    soft_pillow = '安眠枕',
    feather_mattress = '羽毛被',
    dumbbell = '哑铃',
    iron_sandals = '铁木屐',

    sex_items = 'H类(&S)',
    condom = '安全套',
    vibrator = '跳蛋',
    dildo = '按摩棒',
    anal_dildo = '菊穴按摩棒',
}

local function draw()
    for _, item_key in ipairs(item_sort) do
        local items = pointer.item_list[item_key]
        local item_caption = item_caption_map[item_key]
        local MenuSubItem = createMenuItem(ItemMenu)
        MenuSubItem.caption = item_caption

        for _, sub_item_key in ipairs(sub_item_sort[item_key]) do
            local sub_item_caption = item_caption_map[sub_item_key]
            local MenuSubSubItem = createMenuItem(MenuSubItem)

            MenuSubSubItem.caption = sub_item_caption
            MenuSubSubItem.onClick = function()
                util.prompt(sub_item_caption, items[sub_item_key])
            end

            MenuSubItem.add(MenuSubSubItem)
        end
        ItemMenu.add(MenuSubItem)
    end

    -- Help Documentation
    local HelpDocumentationMenu = createMenuItem(MenuHelp)
    HelpDocumentationMenu.caption = '文档(&D)'
    HelpDocumentationMenu.onClick = function()
        shellExecute(DOCUMENTATION_URL)
    end

    -- Help About
    local HelpAboutMenu = createMenuItem(MenuHelp)
    HelpAboutMenu.caption = '关于(&A)'
    HelpAboutMenu.onClick = function()
        showMessage(ABOUT_TEXT)
    end

    HelpMenu.add(HelpDocumentationMenu)
    HelpMenu.add(SeparatorMenu)
    HelpMenu.add(HelpAboutMenu)

    MainMenu.Items.add(ItemMenu)
    MainMenu.Items.add(HelpMenu)
end

local function enable()
    ItemMenu.Enabled = true
end

local function disable()
    ItemMenu.Enabled = false
end

return {
    draw = draw,
    enable = enable,
    disable = disable,
}
