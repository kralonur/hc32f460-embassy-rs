#[doc = "Register `POERB` reader"]
pub type R = crate::R<PoerbSpec>;
#[doc = "Register `POERB` writer"]
pub type W = crate::W<PoerbSpec>;
#[doc = "Field `POUTE00` reader - desc POUTE00"]
pub type Poute00R = crate::BitReader;
#[doc = "Field `POUTE00` writer - desc POUTE00"]
pub type Poute00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE01` reader - desc POUTE01"]
pub type Poute01R = crate::BitReader;
#[doc = "Field `POUTE01` writer - desc POUTE01"]
pub type Poute01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE02` reader - desc POUTE02"]
pub type Poute02R = crate::BitReader;
#[doc = "Field `POUTE02` writer - desc POUTE02"]
pub type Poute02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE03` reader - desc POUTE03"]
pub type Poute03R = crate::BitReader;
#[doc = "Field `POUTE03` writer - desc POUTE03"]
pub type Poute03W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE04` reader - desc POUTE04"]
pub type Poute04R = crate::BitReader;
#[doc = "Field `POUTE04` writer - desc POUTE04"]
pub type Poute04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE05` reader - desc POUTE05"]
pub type Poute05R = crate::BitReader;
#[doc = "Field `POUTE05` writer - desc POUTE05"]
pub type Poute05W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE06` reader - desc POUTE06"]
pub type Poute06R = crate::BitReader;
#[doc = "Field `POUTE06` writer - desc POUTE06"]
pub type Poute06W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE07` reader - desc POUTE07"]
pub type Poute07R = crate::BitReader;
#[doc = "Field `POUTE07` writer - desc POUTE07"]
pub type Poute07W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE08` reader - desc POUTE08"]
pub type Poute08R = crate::BitReader;
#[doc = "Field `POUTE08` writer - desc POUTE08"]
pub type Poute08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE09` reader - desc POUTE09"]
pub type Poute09R = crate::BitReader;
#[doc = "Field `POUTE09` writer - desc POUTE09"]
pub type Poute09W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE10` reader - desc POUTE10"]
pub type Poute10R = crate::BitReader;
#[doc = "Field `POUTE10` writer - desc POUTE10"]
pub type Poute10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE11` reader - desc POUTE11"]
pub type Poute11R = crate::BitReader;
#[doc = "Field `POUTE11` writer - desc POUTE11"]
pub type Poute11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE12` reader - desc POUTE12"]
pub type Poute12R = crate::BitReader;
#[doc = "Field `POUTE12` writer - desc POUTE12"]
pub type Poute12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE13` reader - desc POUTE13"]
pub type Poute13R = crate::BitReader;
#[doc = "Field `POUTE13` writer - desc POUTE13"]
pub type Poute13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE14` reader - desc POUTE14"]
pub type Poute14R = crate::BitReader;
#[doc = "Field `POUTE14` writer - desc POUTE14"]
pub type Poute14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE15` reader - desc POUTE15"]
pub type Poute15R = crate::BitReader;
#[doc = "Field `POUTE15` writer - desc POUTE15"]
pub type Poute15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POUTE00"]
    #[inline(always)]
    pub fn poute00(&self) -> Poute00R {
        Poute00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POUTE01"]
    #[inline(always)]
    pub fn poute01(&self) -> Poute01R {
        Poute01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POUTE02"]
    #[inline(always)]
    pub fn poute02(&self) -> Poute02R {
        Poute02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc POUTE03"]
    #[inline(always)]
    pub fn poute03(&self) -> Poute03R {
        Poute03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc POUTE04"]
    #[inline(always)]
    pub fn poute04(&self) -> Poute04R {
        Poute04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc POUTE05"]
    #[inline(always)]
    pub fn poute05(&self) -> Poute05R {
        Poute05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc POUTE06"]
    #[inline(always)]
    pub fn poute06(&self) -> Poute06R {
        Poute06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc POUTE07"]
    #[inline(always)]
    pub fn poute07(&self) -> Poute07R {
        Poute07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc POUTE08"]
    #[inline(always)]
    pub fn poute08(&self) -> Poute08R {
        Poute08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc POUTE09"]
    #[inline(always)]
    pub fn poute09(&self) -> Poute09R {
        Poute09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc POUTE10"]
    #[inline(always)]
    pub fn poute10(&self) -> Poute10R {
        Poute10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc POUTE11"]
    #[inline(always)]
    pub fn poute11(&self) -> Poute11R {
        Poute11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc POUTE12"]
    #[inline(always)]
    pub fn poute12(&self) -> Poute12R {
        Poute12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc POUTE13"]
    #[inline(always)]
    pub fn poute13(&self) -> Poute13R {
        Poute13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POUTE14"]
    #[inline(always)]
    pub fn poute14(&self) -> Poute14R {
        Poute14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POUTE15"]
    #[inline(always)]
    pub fn poute15(&self) -> Poute15R {
        Poute15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POUTE00"]
    #[inline(always)]
    pub fn poute00(&mut self) -> Poute00W<'_, PoerbSpec> {
        Poute00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POUTE01"]
    #[inline(always)]
    pub fn poute01(&mut self) -> Poute01W<'_, PoerbSpec> {
        Poute01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POUTE02"]
    #[inline(always)]
    pub fn poute02(&mut self) -> Poute02W<'_, PoerbSpec> {
        Poute02W::new(self, 2)
    }
    #[doc = "Bit 3 - desc POUTE03"]
    #[inline(always)]
    pub fn poute03(&mut self) -> Poute03W<'_, PoerbSpec> {
        Poute03W::new(self, 3)
    }
    #[doc = "Bit 4 - desc POUTE04"]
    #[inline(always)]
    pub fn poute04(&mut self) -> Poute04W<'_, PoerbSpec> {
        Poute04W::new(self, 4)
    }
    #[doc = "Bit 5 - desc POUTE05"]
    #[inline(always)]
    pub fn poute05(&mut self) -> Poute05W<'_, PoerbSpec> {
        Poute05W::new(self, 5)
    }
    #[doc = "Bit 6 - desc POUTE06"]
    #[inline(always)]
    pub fn poute06(&mut self) -> Poute06W<'_, PoerbSpec> {
        Poute06W::new(self, 6)
    }
    #[doc = "Bit 7 - desc POUTE07"]
    #[inline(always)]
    pub fn poute07(&mut self) -> Poute07W<'_, PoerbSpec> {
        Poute07W::new(self, 7)
    }
    #[doc = "Bit 8 - desc POUTE08"]
    #[inline(always)]
    pub fn poute08(&mut self) -> Poute08W<'_, PoerbSpec> {
        Poute08W::new(self, 8)
    }
    #[doc = "Bit 9 - desc POUTE09"]
    #[inline(always)]
    pub fn poute09(&mut self) -> Poute09W<'_, PoerbSpec> {
        Poute09W::new(self, 9)
    }
    #[doc = "Bit 10 - desc POUTE10"]
    #[inline(always)]
    pub fn poute10(&mut self) -> Poute10W<'_, PoerbSpec> {
        Poute10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc POUTE11"]
    #[inline(always)]
    pub fn poute11(&mut self) -> Poute11W<'_, PoerbSpec> {
        Poute11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc POUTE12"]
    #[inline(always)]
    pub fn poute12(&mut self) -> Poute12W<'_, PoerbSpec> {
        Poute12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc POUTE13"]
    #[inline(always)]
    pub fn poute13(&mut self) -> Poute13W<'_, PoerbSpec> {
        Poute13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POUTE14"]
    #[inline(always)]
    pub fn poute14(&mut self) -> Poute14W<'_, PoerbSpec> {
        Poute14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POUTE15"]
    #[inline(always)]
    pub fn poute15(&mut self) -> Poute15W<'_, PoerbSpec> {
        Poute15W::new(self, 15)
    }
}
#[doc = "desc POERB\n\nYou can [`read`](crate::Reg::read) this register and get [`poerb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poerb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PoerbSpec;
impl crate::RegisterSpec for PoerbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`poerb::R`](R) reader structure"]
impl crate::Readable for PoerbSpec {}
#[doc = "`write(|w| ..)` method takes [`poerb::W`](W) writer structure"]
impl crate::Writable for PoerbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POERB to value 0"]
impl crate::Resettable for PoerbSpec {}
