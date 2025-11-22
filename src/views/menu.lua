local config = require('config')
local pointer = require('pointer')
local util = require('util')

local MainMenu = createMainMenu(Window)
local SeparatorMenu = createMenuItem(MainMenu)
local ItemMenu = createMenuItem(MainMenu)
local HelpMenu = createMenuItem(MainMenu)

local ABOUT_TEXT = string.format([[Version %s

Powered by Cheat Engine
Copyright © 2025 by Yuki<admin@yuki.sh>]], config.version)
local DOCUMENTATION_URL = 'https://blog.yuki.sh/posts/05064242ec86/'

local category_list = {
    'drinks',
    'presents',
    'books',
    'household_items',
    'sex_items',
}
local prop_list = {
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
local category_text_map = {
    drinks = '茶水类(&D)',
    presents = '礼物类(&P)',
    books = '书籍类(&B)',
    household_items = '家具类(&H)',
    sex_items = 'H类(&S)',
}
local item_text_map = {
    green_tea = '绿茶',
    coffee = '咖啡',
    black_tea = '红茶',
    sleep_tea = '安眠茶',
    stimulant = '壮阳茶',
    love_potion = '媚药',
    after_pill = '避孕药',

    sweets = '零食',
    present = '礼物',

    adventure_book = '冒险的书',
    naughty_book = '色色的书',
    doujinshi = '小薄本',

    soft_pillow = '安眠枕',
    feather_mattress = '羽毛被',
    dumbbell = '哑铃',
    iron_sandals = '铁木屐',

    condom = '安全套',
    vibrator = '跳蛋',
    dildo = '按摩棒',
    anal_dildo = '菊穴按摩棒',
}
local prompt_item_list = {
    'green_tea',
    'coffee',
    'black_tea',
    'sleep_tea',
    'stimulant',
    'love_potion',
    'after_pill',
    'sweets',
    'present',
    'adventure_book',
    'naughty_book',
    'doujinshi',
    'condom',
}

local function createPromptMenuItem(owner, key, offsets)
    local text = item_text_map[key]
    local Item = createMenuItem(owner)

    Item.caption = text
    Item.onClick = function()
        util.prompt(text, offsets)
    end
    owner.add(Item)
end

local function createCheckMenuItem(owner, key, offsets)
    local text = item_text_map[key]
    local Item = createMenuItem(owner)

    Item.autoCheck = true
    Item.caption = text
    Item.onClick = function()
        util.setPointerValue(offsets, Item.checked and 1 or 0)
    end
    owner.add(Item)
end

local function updateItemCount(sender)
    for category_index, category in ipairs(category_list) do
        local CategoryItem = sender.Item[category_index - 1]

        for prop_index, prop in ipairs(prop_list[category]) do
            local PropItem = CategoryItem.Item[prop_index - 1]
            local offsets = pointer.item_list[category][prop]
            local prop_text = item_text_map[prop]
            local prop_count = util.getPointerValue(offsets)

            if (util.includes(prompt_item_list, prop)) then
                PropItem.caption = prop_text .. '\tx' .. prop_count
            else
                PropItem.checked = prop_count > 0
            end
        end
    end
end

local function createItemMenu()
    ItemMenu.caption = '道具(&I)'
    ItemMenu.enabled = false
    ItemMenu.onClick = updateItemCount

    for _, category in ipairs(category_list) do
        local category_text = category_text_map[category]
        local CategoryItem = createMenuItem(ItemMenu)

        CategoryItem.setCaption(category_text)

        for _, prop in ipairs(prop_list[category]) do
            local offsets = pointer.item_list[category][prop]

            if (util.includes(prompt_item_list, prop)) then
                createPromptMenuItem(CategoryItem, prop, offsets)
            else
                createCheckMenuItem(CategoryItem, prop, offsets)
            end
        end
        ItemMenu.add(CategoryItem)
    end
    MainMenu.Items.add(ItemMenu)
end

local function createHelpMenu()
    local DocumentationMenu = createMenuItem(MenuHelp)
    local AboutMenu = createMenuItem(MenuHelp)

    HelpMenu.caption = '帮助(&H)'
    DocumentationMenu.caption = '文档(&D)'
    AboutMenu.caption = '关于(&A)'

    DocumentationMenu.onClick = function() shellExecute(DOCUMENTATION_URL) end
    AboutMenu.onClick = function() showMessage(ABOUT_TEXT) end

    HelpMenu.add(DocumentationMenu)
    HelpMenu.add(SeparatorMenu)
    HelpMenu.add(AboutMenu)
    MainMenu.Items.add(HelpMenu)
end

local function draw()
    SeparatorMenu.setCaption('-')
    createItemMenu()
    createHelpMenu()
end

local function attach()
    ItemMenu.enabled = true
end

local function detach()
    ItemMenu.enabled = false
end

return {
    draw = draw,
    attach = attach,
    detach = detach,
}
