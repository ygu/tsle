require 'webspicy'

Webspicy::Configuration.new(Path.dir) do |c|
  c.host = "http://localhost:8080"
end
