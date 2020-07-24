FROM debian

## Install Base Packages
RUN apt-get update && apt-get -y install \
    apache2 \
    make \
    curl \
    git \
    gcc

RUN a2enmod rewrite
RUN a2enmod cgid

RUN ln -sf /proc/self/fd/1 /var/log/apache2/access.log && \
    ln -sf /proc/self/fd/1 /var/log/apache2/error.log && \
    ln -sf /proc/self/fd/1 /var/log/apache2/other_vhosts_access.log


RUN echo '<VirtualHost *:80> \n\
   ServerAdmin webmaster@myawesomesite.com \n\
   DocumentRoot /var/www/html/ \n\
   Options +ExecCGI \n\
   DirectoryIndex web \n\
   <FilesMatch "^[^\.]+$"> \n\
     SetHandler cgi-script \n\
   </FilesMatch> \n\
   <Directory "/var/www/html/"> \n\
       Options +ExecCGI \n\
       <FilesMatch "^[^\.]+$"> \n\
         SetHandler cgi-script \n\
       </FilesMatch> \n\
       <IfModule mod_rewrite.c> \n\
           RewriteEngine On \n\
           RewriteBase / \n\
           RewriteRule ^web$ - [L] \n\
           RewriteCond %{REQUEST_FILENAME} !-f \n\
           RewriteCond %{REQUEST_FILENAME} !-d \n\
           RewriteRule . /web [L] \n\
        </IfModule> \n\
   </Directory> \n\
</VirtualHost> \n\
' > /etc/apache2/sites-available/000-default.conf

EXPOSE 80

CMD /usr/sbin/apache2ctl -D FOREGROUND