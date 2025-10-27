local config = require("config")
local cover = require("views.cover")

local width, height = config.window.width - cover.width, config.window.height

local function create(owner)
    local Feature = createListView(owner)

    Feature.Width, Feature.Height = width, height
    Feature.Left = cover.width
    Feature.BorderStyle = bsNone
    Feature.RowSelect = true
    Feature.ReadOnly = true
    Feature.Checkboxes = true
    Feature.OnClick = function(sender)
        local Item = sender.Selected

        if Item ~= nil then
            Item.setChecked(not Item.Checked)
        end
    end

    return Feature
end

return {
    create = create,
}
