#[doc = "Register `PWMLV` reader"]
pub type R = crate::R<PwmlvSpec>;
#[doc = "Register `PWMLV` writer"]
pub type W = crate::W<PwmlvSpec>;
#[doc = "Field `PWMLV0` reader - desc PWMLV0"]
pub type Pwmlv0R = crate::BitReader;
#[doc = "Field `PWMLV0` writer - desc PWMLV0"]
pub type Pwmlv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMLV1` reader - desc PWMLV1"]
pub type Pwmlv1R = crate::BitReader;
#[doc = "Field `PWMLV1` writer - desc PWMLV1"]
pub type Pwmlv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMLV2` reader - desc PWMLV2"]
pub type Pwmlv2R = crate::BitReader;
#[doc = "Field `PWMLV2` writer - desc PWMLV2"]
pub type Pwmlv2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PWMLV0"]
    #[inline(always)]
    pub fn pwmlv0(&self) -> Pwmlv0R {
        Pwmlv0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PWMLV1"]
    #[inline(always)]
    pub fn pwmlv1(&self) -> Pwmlv1R {
        Pwmlv1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PWMLV2"]
    #[inline(always)]
    pub fn pwmlv2(&self) -> Pwmlv2R {
        Pwmlv2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PWMLV0"]
    #[inline(always)]
    pub fn pwmlv0(&mut self) -> Pwmlv0W<'_, PwmlvSpec> {
        Pwmlv0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc PWMLV1"]
    #[inline(always)]
    pub fn pwmlv1(&mut self) -> Pwmlv1W<'_, PwmlvSpec> {
        Pwmlv1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc PWMLV2"]
    #[inline(always)]
    pub fn pwmlv2(&mut self) -> Pwmlv2W<'_, PwmlvSpec> {
        Pwmlv2W::new(self, 2)
    }
}
#[doc = "desc PWMLV\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmlv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmlv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmlvSpec;
impl crate::RegisterSpec for PwmlvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmlv::R`](R) reader structure"]
impl crate::Readable for PwmlvSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmlv::W`](W) writer structure"]
impl crate::Writable for PwmlvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMLV to value 0"]
impl crate::Resettable for PwmlvSpec {}
