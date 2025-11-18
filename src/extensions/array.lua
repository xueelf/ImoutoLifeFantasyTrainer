function table.isArray(list)
    return type(list) == 'table' and #list
end

function table.includes(list, value)
    for _, item in ipairs(list) do
        if item == value then
            return true
        end
    end
    return false
end
