function string.endsWith(search, suffix)
    if #suffix > #search then
        return false
    end
    return search:sub(- #suffix) == suffix
end
