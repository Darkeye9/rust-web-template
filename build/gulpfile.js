var gulp = require('gulp');
var jshint = require('gulp-jshint');
var less = require('gulp-less');
var autoprefixer = require('gulp-autoprefixer');
var cleanCSS = require('gulp-clean-css');
var path = require('path');
var htmlmin = require('gulp-htmlmin');
var runSequence = require('run-sequence');
var changed = require('gulp-changed');
var imagemin = require('gulp-imagemin');
var shell = require('gulp-shell');
var concat = require('gulp-concat');
var uglify = require('gulp-uglify');
var semantic_build = require('./node_modules/semantic-ui/tasks/build');

gulp.task('less', function () {
  return gulp.src('./less/main.less')
    .pipe(less({
      paths: [ path.join(__dirname, 'less', 'includes') ],
    }))
    .pipe(autoprefixer({
        browsers: ['last 10 versions'],
        cascade: false
    }))
    .pipe(cleanCSS({compatibility: 'ie8'}))
    .pipe(gulp.dest('./output/static/css'));
});

gulp.task('jshint', function() {
  return gulp.src('./js/**/*.js')
    .pipe(jshint())
    .pipe(jshint.reporter('default'));
});

gulp.task('scripts', ['jshint'], function() {
  return gulp.src([
      './node_modules/jquery/dist/jquery.js',
      './node_modules/semantic-ui/dist/semantic.js',
      './js/**/*.js'])
    .pipe(concat('main.js'))
    //.pipe(stripDebug())
    .pipe(uglify())
    .pipe(gulp.dest('./output/static/js'));
});

gulp.task('html', function() {
  var htmlSrc = './html/**/*.html',
      htmlDst = './output/private/html';

  return gulp.src(htmlSrc)
    //.pipe(changed(htmlDst))
    .pipe(htmlmin({collapseWhitespace: true}))
    .pipe(gulp.dest(htmlDst));
});

gulp.task('imagemin', function() {
  var imgSrc = './images/**/*',
      imgDst = './output/static/images';

  gulp.src(imgSrc)
    .pipe(changed(imgDst))
    .pipe(imagemin())
    .pipe(gulp.dest(imgDst));
});

gulp.task('copy_semantic', function() {
    return gulp.src(['./node_modules/semantic-ui/dist/semantic.css'])
    .pipe(gulp.dest('./output/static/css/'));
});

gulp.task('copy_fonts', function() {
    return gulp.src('./fonts/**/*')
    .pipe(gulp.dest('./output/static/fonts/'));
});

gulp.task('copy_static', function() {
    return gulp.src('./output/static/**/*')
    .pipe(gulp.dest('../public/static/'));
});

gulp.task('copy_private', function() {
   gulp.src('./output/private/html/**/*')
   .pipe(gulp.dest('../private/templates/'));
});

gulp.task('build_semantic', semantic_build);

gulp.task('copy_semantic_site', function() {
   gulp.src('./sm_site/**/*')
   .pipe(gulp.dest('node_modules/semantic-ui/src/site'));
});

gulp.task('default', function(callback) {
  runSequence(['html','less', 'scripts', 'copy_semantic', 'copy_fonts', 'imagemin'],
              ['copy_static', 'copy_private'],
              callback);
});

gulp.task('watch', ['default'] , function(){
  gulp.watch(['./js/**/*.js', './html/**/*.html', './less/**/*.less'], function() {
    gulp.run('default');
  });
});

gulp.task('semantic', function(callback) {
  runSequence(['copy_semantic_site'],
              ['build_semantic'],
              ['copy_semantic'],
              ['copy_static'],
              callback);
});
