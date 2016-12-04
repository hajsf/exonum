var express = require('express');
var request = require('request');
var router = express.Router();

function render(req, res, next, view) {
    var hash = req.param('hash');

    request.get('http://exonum.com/backends/timestamping/info/' + hash, function(error, response, body) {
        if (!error) {
            if (response.statusCode === 200) {
                var data = JSON.parse(body);

                data['title'] = 'Certificate of proof';
                data['url'] = encodeURIComponent('http://' + req.headers.host + '/f/' + hash);
                data['file_path'] = 'http://exonum.com/backends/timestamping/content/' + hash;

                res.render(view, data);
            } else if (response.statusCode === 400) {
                res.render('file-not-found', {title: 'File not found'});
            } else {
                res.render('error');
            }
        } else {
            res.render('error');
        }
    });
}

router.get('/:hash', function() {
    var arguments = [].slice.call(arguments, 0);
    arguments.push('file');
    render.apply(this, arguments)
});

router.get('/:hash/success', function() {
    var arguments = [].slice.call(arguments, 0);
    arguments.push('success');
    render.apply(this, arguments)
});

module.exports = router;
