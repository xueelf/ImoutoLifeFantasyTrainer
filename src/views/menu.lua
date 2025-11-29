local util = require('util')
local app = require('config.app')
local menu = require('config.menu')
local pointer = require('config.pointer')

local Menu = createMainMenu(Window)
local SeparatorMenu = createMenuItem(Menu)

local ABOUT_TEXT = string.format([[Version %s

Powered by Cheat Engine
Copyright © 2025 by Yuki<admin@yuki.sh>]], app.version)
local DOCUMENTATION_URL = 'https://blog.yuki.sh/posts/05064242ec86/'

local function prompt(title, offsets)
    local input = util.prompt(title, offsets)

    if input ~= nil then
        util.playVoice('Enter')
    else
        util.playVoice('Cancel')
    end
end

local function check(offsets, checked)
    if checked then
        util.setPointerValue(offsets, 1)
        util.playVoice('Enter')
    else
        util.setPointerValue(offsets, 0)
        util.playVoice('Cancel')
    end
end

local handler = {
    documentation = function()
        shellExecute(DOCUMENTATION_URL)
    end,
    about = function()
        showMessage(ABOUT_TEXT)
    end,
}

local function updateMenuStatus(sender, list)
    for category_index, category in ipairs(list) do
        local Category = sender.Item[category_index - 1]

        for prop_index, prop in ipairs(category) do
            if prop.type == 'action' then
                break
            end
            local Prop = Category.Item[prop_index - 1]
            local offsets = pointer[list.key][category.key][prop.key]
            local count = util.getPointerValue(offsets)

            if prop.type == 'prompt' then
                local format = category.format or prop.format or count
                Prop.caption = prop.caption .. '\t' .. string.gsub(format, '{}', count)
            elseif prop.type == 'check' then
                Prop.checked = count > 0
            elseif prop.type == 'select' then
                for i = 0, Prop.count - 1 do
                    Prop.Item[i].checked = prop.options[i + 1].value == count
                end
            end
        end
    end
end

local function createMenu(owner, list, ref)
    local is_main = owner.className == 'TMainMenu'

    for _, value in ipairs(list) do
        local MenuItem = createMenuItem(value)

        MenuItem.setCaption(value.caption)

        if is_main then
            owner.Items.add(MenuItem)

            MenuItem.enabled = value.enabled
            MenuItem.onClick = function(sender)
                updateMenuStatus(sender, value)
            end
        else
            owner.add(MenuItem)
        end

        if value.type and ref then
            local offsets = ref[value.key]

            if value.type == 'action' then
                MenuItem.onClick = handler[value.key]
            elseif value.type == 'prompt' then
                MenuItem.onClick = function() prompt(value.caption, offsets) end
            elseif value.type == 'check' then
                MenuItem.autoCheck = true
                MenuItem.onClick = function() check(offsets, MenuItem.checked) end
            elseif value.type == 'select' then
                for _, option in ipairs(value.options) do
                    local OptionItem = createMenuItem(option)

                    OptionItem.setCaption(option.caption)
                    OptionItem.onClick = function()
                        for i = 0, MenuItem.count - 1 do
                            MenuItem.Item[i].checked = false
                        end
                        OptionItem.checked = true
                        util.setPointerValue(offsets, option.value)
                        util.playVoice('Enter')
                    end
                    MenuItem.add(OptionItem)
                end
            end
        end
        createMenu(MenuItem, value, ref[value.key] or {})
    end
end

local function draw()
    createMenu(Menu, menu, pointer)
end

local function attach()
    for index = 0, Menu.Items.count - 1 do
        Menu.Items[index].enabled = true
    end
end

local function detach()
    for index = 0, Menu.Items.count - 1 do
        Menu.Items[index].enabled = menu[index + 1].enabled
    end
end

return {
    draw = draw,
    attach = attach,
    detach = detach,
}
