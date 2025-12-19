local util = require('util')
local app = require('config.app')
local navigate = require('config.navigate')
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

local function syncMenuStatus(sender, list, ref)
    for i = 0, sender.count - 1 do
        local MenuItem = sender.Item[i]
        local nav = list[i + 1]

        if #nav > 0 then
            syncMenuStatus(MenuItem, nav, ref[nav.key])
        else
            if nav.type == 'action' then
                break
            end
            local offsets = ref[nav.key]
            local count = util.getPointerValue(offsets)

            if nav.type == 'prompt' then
                local format = nav.format or count
                MenuItem.caption = nav.caption .. '\t' .. string.gsub(format, '{}', count)
            elseif nav.type == 'check' then
                MenuItem.checked = count > 0
            elseif nav.type == 'select' then
                for j = 0, MenuItem.count - 1 do
                    MenuItem.Item[j].checked = nav.options[j + 1].value == count
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
                syncMenuStatus(sender, value, ref[value.key])
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

                    if option.readonly then
                        OptionItem.enabled = false
                    end
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
    createMenu(Menu, navigate, pointer)
end

local function attach()
    for index = 0, Menu.Items.count - 1 do
        Menu.Items[index].enabled = true
    end
end

local function detach()
    for index = 0, Menu.Items.count - 1 do
        Menu.Items[index].enabled = navigate[index + 1].enabled
    end
end

return {
    draw = draw,
    attach = attach,
    detach = detach,
}
